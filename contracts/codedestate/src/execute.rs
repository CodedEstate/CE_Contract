use cw_ownable::OwnershipError;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{Binary, CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, BankMsg, Coin};

use cw721::{ContractInfoResponse, Cw721Execute, Cw721ReceiveMsg,Landlord,Tenant, LongTermRental, Expiration};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Approval, Cw721Contract, TokenInfo};

impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response<C>> {
        let info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        self.contract_info.save(deps.storage, &info)?;

        cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.minter))?;

        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<T, E>,
    ) -> Result<Response<C>, ContractError> {
        match msg {
            ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => self.mint(deps, info, token_id, owner, token_uri, extension),

            // ExecuteMsg::SetPrice{
            //     token_id,
            //     price,
            // } => self.setprice(deps, env, info, token_id, price),

            ExecuteMsg::SetMetadata{
                token_id,
                token_uri,
            } => self.setmetadata(deps, env, info, token_id, token_uri),

            ExecuteMsg::SetListForLongTermRental{
                token_id,
                islisted,
                denom,
                price_per_month,
                refundable_deposit,
                available_period,
            } => self.setlistforlongtermrental(deps, env, info, token_id, islisted,denom, price_per_month, refundable_deposit, available_period),


            

            ExecuteMsg::SetUnlistForLongtermRental {
                token_id,
            } => self.setunlistforlongtermrental(
                deps,
                env,
                info,
                token_id
            ),

            ExecuteMsg::RejectReservationForLongterm {
                token_id,
            } => self.rejectreservationforlongterm(
                deps,
                env,
                info,
                token_id
            ),
            

            ExecuteMsg::SetReservationForLongTerm{
                token_id,
                isreserved,
                deposit_amount,
                deposit_denom,
                renting_period,
            } => self.setreservationforlongterm(deps,info, token_id, isreserved, deposit_amount,deposit_denom, renting_period),

            ExecuteMsg::CancelReservationForLongterm {
                token_id,
            } => self.cancelreservationforlongterm(deps, info, token_id),

            ExecuteMsg::ProceedLongtermRental {
                token_id,
            } => self.proceedlongtermrental(deps, env, info, token_id),

            ExecuteMsg::SetEjariForLongTermRental{
                token_id,
                ejari,
            } => self.setejariforlongtermrental(deps, env, info, token_id, ejari),

            ExecuteMsg::DepositForLongTermRental {
                token_id,
            } => self.depositforlongtermrental(deps,env, info, token_id),

            ExecuteMsg::WithdrawToLandlord {
                token_id, amount, denom,
            } => self.withdrawtolandlord(deps, env, info, token_id, amount, denom),

            ExecuteMsg::FinalizeLongTermRental {
                token_id,
            } => self.finalizelongtermrental(deps, env, info, token_id), 

            // ExecuteMsg::SetListing{
            //     token_id,
            //     islisted,
            //     price,
            // } => self.setlisting(deps, env, info, token_id, islisted, price),


            // ExecuteMsg::Bid{
            //     token_id,
            //     offer,
            // } => self.bid(deps,  info, token_id, offer),

            // ExecuteMsg::Auction{
            //     token_id,
            //     new_owner,
            // } => self.auction(deps, env, info, token_id, new_owner),

            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => self.approve(deps, env, info, spender, token_id, expires),
            ExecuteMsg::Revoke { spender, token_id } => {
                self.revoke(deps, env, info, spender, token_id)
            }
            ExecuteMsg::ApproveAll { operator, expires } => {
                self.approve_all(deps, env, info, operator, expires)
            }
            ExecuteMsg::RevokeAll { operator } => self.revoke_all(deps, env, info, operator),
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => self.transfer_nft(deps, env, info, recipient, token_id),
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => self.send_nft(deps, env, info, contract, token_id, msg),
            ExecuteMsg::Burn { token_id } => self.burn(deps, env, info, token_id),
            ExecuteMsg::UpdateOwnership(action) => Self::update_ownership(deps, env, info, action),
            ExecuteMsg::Extension { msg: _ } => Ok(Response::default()),
        }
    }
}

// TODO pull this into some sort of trait extension??
impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn mint(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: T,
    ) -> Result<Response<C>, ContractError> {

        // cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let longterm_rental = LongTermRental{
            islisted:None,
            isreserved:None,
            landlord:None,
            tenant:None,
            tenant_address:None,
            deposit_amount:Uint128::from(0u64),
            withdrawn_amount:Uint128::from(0u64),
            renting_flag:None,
            ejari_flag:None,
        };
        // create the token
        let token = TokenInfo {
            owner: deps.api.addr_validate(&owner)?,
            approvals: vec![],
            // mode:None,
            longterm_rental:longterm_rental,
            // islisted:false,
            // price:0,
            // bids:vec![],
            token_uri,
            extension,
        };
        self.tokens
            .update(deps.storage, &token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;

        self.increment_tokens(deps.storage)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("token_id", token_id))
    }

    pub fn update_ownership(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        action: cw_ownable::Action,
    ) -> Result<Response<C>, ContractError> {
        let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
        Ok(Response::new().add_attributes(ownership.into_attributes()))
    }
}

impl<'a, T, C, E, Q> Cw721Execute<T, C> for Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    type Err = ContractError;

    fn transfer_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        self._transfer_nft(deps, &env, &info, &recipient, &token_id)?;

        Ok(Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", recipient)
            .add_attribute("token_id", token_id))
    }

    fn send_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<C>, ContractError> {
        // Transfer token
        self._transfer_nft(deps, &env, &info, &contract, &token_id)?;

        let send = Cw721ReceiveMsg {
            sender: info.sender.to_string(),
            token_id: token_id.clone(),
            msg,
        };

        // Send message
        Ok(Response::new()
            .add_message(send.into_cosmos_msg(contract.clone())?)
            .add_attribute("action", "send_nft")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", contract)
            .add_attribute("token_id", token_id))
    }

    fn approve(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        self._update_approvals(deps, &env, &info, &spender, &token_id, true, expires)?;

        Ok(Response::new()
            .add_attribute("action", "approve")
            .add_attribute("sender", info.sender)
            .add_attribute("spender", spender)
            .add_attribute("token_id", token_id))
    }



    fn revoke(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        self._update_approvals(deps, &env, &info, &spender, &token_id, false, None)?;

        Ok(Response::new()
            .add_attribute("action", "revoke")
            .add_attribute("sender", info.sender)
            .add_attribute("spender", spender)
            .add_attribute("token_id", token_id))
    }

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(&env.block) {
            return Err(ContractError::Expired {});
        }

        // set the operator for us
        let operator_addr = deps.api.addr_validate(&operator)?;
        self.operators
            .save(deps.storage, (&info.sender, &operator_addr), &expires)?;

        Ok(Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", info.sender)
            .add_attribute("operator", operator))
    }

    fn revoke_all(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response<C>, ContractError> {
        let operator_addr = deps.api.addr_validate(&operator)?;
        self.operators
            .remove(deps.storage, (&info.sender, &operator_addr));

        Ok(Response::new()
            .add_attribute("action", "revoke_all")
            .add_attribute("sender", info.sender)
            .add_attribute("operator", operator))
    }

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        let token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_send(deps.as_ref(), &env, &info, &token)?;

        self.tokens.remove(deps.storage, &token_id)?;
        self.decrement_tokens(deps.storage)?;

        Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }
}

// helpers
impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn _transfer_nft(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        recipient: &str,
        token_id: &str,
    ) -> Result<TokenInfo<T>, ContractError> {
        let mut token = self.tokens.load(deps.storage, token_id)?;
        // ensure we have permissions
        self.check_can_send(deps.as_ref(), env, info, &token)?;
        // set owner and remove existing approvals
        token.owner = deps.api.addr_validate(recipient)?;
        token.approvals = vec![];
        self.tokens.save(deps.storage, token_id, &token)?;
        Ok(token)
    }

    // pub fn setprice(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     token_id: String,
    //     price:u64,
    //     // expires: Option<Expiration>,
    // ) -> Result<Response<C>, ContractError> {
    //     let mut token = self.tokens.load(deps.storage, &token_id)?;
    //     // ensure we have permissions
    //     self.check_can_send(deps.as_ref(), &env, &info, &token)?;

    //     token.price = price;
    //     // token.islisted = true;
    //     self.tokens.save(deps.storage, &token_id, &token)?;

    //     // let spender_addr = env.contract.address.clone();
    //     // let spender = spender_addr.to_string();
        
    //     // self._update_approvals(deps, &env, &info, &spender, &token_id, true, expires)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "setprice")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("token_id", token_id))
    // }

    // pub fn setlisting(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     token_id: String,
    //     islisted:bool,
    //     price:u64,
    //     // expires: Option<Expiration>,
    // ) -> Result<Response<C>, ContractError> {
    //     let mut token = self.tokens.load(deps.storage, &token_id)?;
    //     // ensure we have permissions
    //     self.check_can_send(deps.as_ref(), &env, &info, &token)?;

    //     // token.price = price;
    //     token.islisted = islisted;
    //     token.price = price;
    //     self.tokens.save(deps.storage, &token_id, &token)?;

    //     // let spender_addr = env.contract.address.clone();
    //     // let spender = spender_addr.to_string();
        
    //     // self._update_approvals(deps, &env, &info, &spender, &token_id, true, expires)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "setlisting")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("token_id", token_id))
    // }

    pub fn setmetadata(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        token_uri:String,
        // expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        token.token_uri = Some(token_uri);
        self.tokens.save(deps.storage, &token_id, &token)?;

        // let spender_addr = env.contract.address.clone();
        // let spender = spender_addr.to_string();
        
        // self._update_approvals(deps, &env, &info, &spender, &token_id, true, expires)?;

        Ok(Response::new()
            .add_attribute("action", "setmetadata")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setlistforlongtermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id:String,
        islisted:bool,
        denom:String,
        price_per_month: u64,
        refundable_deposit: u64,
        available_period:Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        let landlord = Landlord{
            denom:denom,
            price_per_month:price_per_month,
            refundable_deposit:refundable_deposit,
            available_period:available_period
        };

        token.longterm_rental.islisted = Some(islisted);
        token.longterm_rental.landlord = Some(landlord);
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setlistforlongtermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setunlistforlongtermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id:String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        token.longterm_rental.islisted = None;
        token.longterm_rental.landlord = None;
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setunlistforlongtermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setreservationforlongterm(
        &self,
        deps: DepsMut,
        // env: Env,
        info: MessageInfo,
        token_id:String,
        _isreserved:bool,
        deposit_amount:u64,
        deposit_denom:String,
        renting_period:Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // let sent_amount = info.funds[0].amount;
        let tenant = Tenant{
            deposit_amount:deposit_amount,
            deposit_denom:deposit_denom,
            renting_period:renting_period,  
        };
        token.longterm_rental.isreserved = Some(true);
        token.longterm_rental.tenant_address = Some(info.sender.clone());
        token.longterm_rental.tenant = Some(tenant);
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setreservationforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

// We use Approve function for long term rental approve for tenant

    pub fn rejectreservationforlongterm(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id:String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        token.longterm_rental.isreserved = None;
        token.longterm_rental.tenant_address = None;
        token.longterm_rental.tenant = None;
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "cancelreservationforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }



    pub fn cancelreservationforlongterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id:String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        let tenant_address = token.longterm_rental.tenant_address.clone();
        match tenant_address {
            Some(address) => {
                if info.sender != address {
                    return Err(ContractError::NotReserved {});
                }
            }
            None => {
                return Err(ContractError::NotReserved {});
            }
        }
        token.longterm_rental.isreserved = None;
        token.longterm_rental.tenant_address = None;
        token.longterm_rental.tenant = None;
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "cancelreservationforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setejariforlongtermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id:String,
        ejari:bool,
    ) -> Result<Response<C>, ContractError>{
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_send(deps.as_ref(), &env, &info, &token)?;
        token.longterm_rental.ejari_flag = Some(ejari);
        self.tokens.save(deps.storage, &token_id, &token)?;
        Ok(Response::new()
        .add_attribute("action", "setejariforlongtermrental")
        .add_attribute("sender", info.sender)
        .add_attribute("token_id", token_id))
    }

    pub fn proceedlongtermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id:String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        let tenant_address = token.longterm_rental.tenant_address.clone();
        match tenant_address {
            Some(address) => {
                if info.sender != address {
                    return Err(ContractError::NotReserved {});
                }

            }
            None => {
                return Err(ContractError::NotReserved {});
            }
        }
        self.check_can_send(deps.as_ref(), &env, &info, &token)?;
        token.longterm_rental.renting_flag = Some(true);
        self.tokens.save(deps.storage, &token_id, &token)?;
        Ok(Response::new()
        .add_attribute("action", "proceedlongtermrental")
        .add_attribute("sender", info.sender)
        .add_attribute("token_id", token_id))
    }

    pub fn depositforlongtermrental(
        &self,
        deps: DepsMut,
        env:Env,
        info: MessageInfo,
        token_id:String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_send(deps.as_ref(), &env, &info, &token)?;
        let _sent_amount = info.funds[0].amount;
        let tenant_address = token.longterm_rental.tenant_address.clone();
        match tenant_address {
            Some(address) => {
                if info.sender != address {
                    return Err(ContractError::NotReserved {});
                }

                if token.longterm_rental.renting_flag != Some(true) {
                    return Err(ContractError::RentalNotActivated {});
                }
            }
            None => {
                return Err(ContractError::NotReserved {});
            }
        }
        token.longterm_rental.deposit_amount += _sent_amount;
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "depositforlongtermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn withdrawtolandlord(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id:String,
        amount: Uint128,
        denom: String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        if token.longterm_rental.deposit_amount < token.longterm_rental.withdrawn_amount + amount {
            return Err(ContractError::UnavailableAmount {});
        }

        if !token.longterm_rental.ejari_flag.is_some() {
            return Err(ContractError::EjariNotConfirmed {});
        }

        token.longterm_rental.withdrawn_amount += amount;
        self.tokens.save(deps.storage, &token_id, &token)?;
        Ok(Response::new()
        .add_attribute("action", "withdrawtolandlord")
        .add_message(BankMsg::Send {
            to_address: token.owner.into_string(),
            amount: vec![Coin{denom, amount}]
        })
    )
    }

    pub fn finalizelongtermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id:String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_send(deps.as_ref(), &env, &info, &token)?;

        token.longterm_rental.isreserved = None;
        token.longterm_rental.tenant = None;
        token.longterm_rental.tenant_address = None;
        token.longterm_rental.deposit_amount = Uint128::from(0u64);
        token.longterm_rental.withdrawn_amount = Uint128::from(0u64);
        token.longterm_rental.renting_flag = None;
        token.longterm_rental.ejari_flag = None;
        self.tokens.save(deps.storage, &token_id, &token)?;

        
        Ok(Response::new()
            .add_attribute("action", "finalizelongtermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    // pub fn bid(
    //     &self,
    //     deps: DepsMut,
    //     info: MessageInfo,
    //     token_id: String,
    //     offer:u64,
    // ) -> Result<Response<C>, ContractError>{
    //     self._update_bids(deps,&info, &token_id, offer)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "bid")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("token_id", token_id))

    // }

    // pub fn auction(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     token_id: String,
    //     new_owner:String,
    // ) -> Result<Response<C>, ContractError>{
    //     // let token = self.tokens.load(deps.storage, &token_id)?;
    //     // let matching_bid = token.bids.iter().find(|bid| bid.buyer == new_owner);
    //     // let offer = matching_bid.map(|bid| bid.offer).unwrap_or(0);
    //     // let _ = self.send_nusd(&new_owner, token.owner.to_string(), offer);
    //     self._transfer_nft(deps, &env, &info, &new_owner, &token_id)?;
    //     Ok(Response::new()
    //     .add_attribute("action", "auction")
    //     .add_attribute("new_owner", new_owner)
    //     .add_attribute("token_id", token_id))
    // }

    // pub fn send_nusd(
    //     &self,
    //     _sender: &str,
    //     recipient: String,
    //     amount: u64,
    // ) -> StdResult<Response> {
    //     let transfer_msg = BankMsg::Send {
    //         // from_address: sender.clone(),
    //         to_address: recipient.clone(),
    //         amount: vec![Coin {
    //             denom: "NUSD".to_string(), // Example token denomination
    //             amount:amount.into(),
    //         }],
    //     };
    
    //     let cosmo_msg = CosmosMsg::Bank(transfer_msg.into());
    
    //     Ok(Response::new().add_message(cosmo_msg))
    // }


    #[allow(clippy::too_many_arguments)]
    pub fn _update_approvals(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        spender: &str,
        token_id: &str,
        // if add == false, remove. if add == true, remove then set with this expiration
        add: bool,
        expires: Option<Expiration>,
    ) -> Result<TokenInfo<T>, ContractError> {
        let mut token = self.tokens.load(deps.storage, token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), env, info, &token)?;

        // update the approval list (remove any for the same spender before adding)
        let spender_addr = deps.api.addr_validate(spender)?;
        token.approvals.retain(|apr| apr.spender != spender_addr);

        // only difference between approve and revoke
        if add {
            // reject expired data as invalid
            let expires = expires.unwrap_or_default();
            if expires.is_expired(&env.block) {
                return Err(ContractError::Expired {});
            }
            let approval = Approval {
                spender: spender_addr,
                expires,
            };
            token.approvals.push(approval);
        }

        self.tokens.save(deps.storage, token_id, &token)?;

        Ok(token)
    }

    // pub fn _update_bids(
    //     &self,
    //     deps: DepsMut,
    //     // env: &Env,
    //     info: &MessageInfo,
    //     token_id: &str,
    //     offer:u64,
    // ) -> Result<TokenInfo<T>, ContractError> {
    //     let mut token = self.tokens.load(deps.storage, token_id)?;

    //     // update the approval list (remove any for the same spender before adding)
    //     let buyer = info.sender.to_string();
    //     token.bids.retain(|apr| apr.buyer != buyer);

    //     let bid = Bid {
    //         buyer: buyer,
    //         offer: offer,
    //     };
    //     token.bids.push(bid);
    //     self.tokens.save(deps.storage, token_id, &token)?;
    //     Ok(token)
    // }






    /// returns true iff the sender can execute approve or reject on the contract
    pub fn check_can_approve(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        token: &TokenInfo<T>,
    ) -> Result<(), ContractError> {
        // owner can approve
        if token.owner == info.sender {
            return Ok(());
        }
        // operator can approve
        let op = self
            .operators
            .may_load(deps.storage, (&token.owner, &info.sender))?;
        match op {
            Some(ex) => {
                if ex.is_expired(&env.block) {
                    Err(ContractError::Ownership(OwnershipError::NotOwner))
                } else {
                    Ok(())
                }
            }
            None => Err(ContractError::Ownership(OwnershipError::NotOwner)),
        }
    }

    /// returns true iff the sender can transfer ownership of the token
    pub fn check_can_send(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        token: &TokenInfo<T>,
    ) -> Result<(), ContractError> {
        // owner can send
        if token.owner == info.sender {
            return Ok(());
        }

        // any non-expired token approval can send
        if token
            .approvals
            .iter()
            .any(|apr| apr.spender == info.sender && !apr.is_expired(&env.block))
        {
            return Ok(());
        }

        // operator can send
        let op = self
            .operators
            .may_load(deps.storage, (&token.owner, &info.sender))?;
        match op {
            Some(ex) => {
                if ex.is_expired(&env.block) {
                    Err(ContractError::Ownership(OwnershipError::NotOwner))
                } else {
                    Ok(())
                }
            }
            None => Err(ContractError::Ownership(OwnershipError::NotOwner)),
        }
    }
}
