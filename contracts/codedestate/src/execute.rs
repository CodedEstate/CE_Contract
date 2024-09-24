// use chrono::NaiveDate;
use cw_ownable::OwnershipError;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{
    Addr, BankMsg, Binary, Coin, CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128
};

use cw721::{
    Bid, CancellationItem, ContractInfoResponse, Cw721Execute, Cw721ReceiveMsg, Expiration, LongTermRental, Rental, Sell, ShortTermRental
};

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

            ExecuteMsg::SetMetadata {
                token_id,
                token_uri,
            } => self.setmetadata(deps, env, info, token_id, token_uri),

            ExecuteMsg::SetExtension {
                token_id,
                extension,
            } => self.setextension(deps, env, info, token_id, extension),

            ExecuteMsg::SetListForSell {islisted, token_id, denom, price, auto_approve } => 
            self.setlistforsell(deps, env, info, islisted,token_id, denom, price, auto_approve),

            ExecuteMsg::SetBidToBuy { token_id } => self.setbidtobuy(deps, env, info, token_id),

            ExecuteMsg::SetListForShortTermRental {
                token_id,
                denom,
                price_per_day,
                auto_approve,
                available_period,
                minimum_stay,
                cancellation,
            } => self.setlistforshorttermrental(
                deps,
                env,
                info,
                token_id,
                denom,
                price_per_day,
                auto_approve,
                available_period,
                minimum_stay,
                cancellation,
            ),

            ExecuteMsg::SetUnlistForShorttermRental { token_id } => {
                self.setunlistforshorttermrental(deps, env, info, token_id)
            }
            ExecuteMsg::SetReservationForShortTerm {
                token_id,
                renting_period,
                guests,
            } => self.setreservationforshortterm(deps, info, token_id, renting_period, guests),
            ExecuteMsg::RejectReservationForShortterm {
                token_id,
                traveler,
                renting_period,
            } => self.rejectreservationforshortterm(
                deps,
                env,
                info,
                token_id,
                traveler,
                renting_period,
            ),
            ExecuteMsg::CancelRentalForShortterm { token_id, renting_period }
            => self.cancelreservationafterapprovalforshortterm(deps, info,env, token_id, renting_period),

            ExecuteMsg::CancelReservationForShortterm {
                token_id,
                renting_period,
            } => self.cancelreservationbeforeapprovalforshortterm(deps, info, token_id, renting_period),

            ExecuteMsg::SetApproveForShortTerm {
                token_id,
                traveler,
                renting_period,
            } => self.setapproveforshortterm(deps, info, env, token_id, renting_period, traveler),

            ExecuteMsg::FinalizeShortTermRental {
                token_id,
                traveler,
                renting_period,
            } => self.finalizeshorttermrental(deps, env, info, token_id, traveler, renting_period),

            ExecuteMsg::SetListForLongTermRental {
                token_id,
                denom,
                price_per_month,
                auto_approve,
                available_period,
                minimum_stay,
                cancellation,
            } => self.setlistforlongtermrental(
                deps,
                env,
                info,
                token_id,
                denom,
                price_per_month,
                auto_approve,
                available_period,
                minimum_stay,
                cancellation
            ),
            
            ExecuteMsg::SetUnlistForLongtermRental { token_id } => {
                self.setunlistforlongtermrental(deps, env, info, token_id)
            }

            ExecuteMsg::SetReservationForLongTerm {
                token_id,
                renting_period,
                guests,
            } => self.setreservationforlongterm(deps, info, token_id, renting_period, guests),

            ExecuteMsg::CancelReservationForLongterm {
                token_id,
                renting_period,
            } => self.cancelreservationbeforeapprovalforlongterm(deps, info, token_id, renting_period),

            ExecuteMsg::CancelRentalForLongterm {
                token_id,
                renting_period,
            } => self.cancelreservationafterapprovalforlongterm(deps, info, token_id, renting_period),

            ExecuteMsg::RejectReservationForLongterm {
                token_id,
                tenant,
                renting_period
            } => self.rejectreservationforlongterm(
                deps,
                env,
                info,
                token_id,
                tenant,
                renting_period
            ),

            ExecuteMsg::DepositForLongTermRental {
                token_id,
                renting_period,
            } => self.depositforlongtermrental(deps, info, token_id, renting_period),

            ExecuteMsg::SetApproveForLongTerm {
                token_id,
                tenant,
                renting_period,
                approved_date,
            } => self.setapproveforlongterm(deps, info, env, token_id, renting_period, tenant, approved_date),
            
            ExecuteMsg::FinalizeLongTermRental {
                token_id,
                tenant,
                renting_period,
            } => self.finalizelongtermrental(deps, env, info, token_id, tenant, renting_period),

            ExecuteMsg::WithdrawToLandlord { token_id, tenant, renting_period, amount, address } => {
                self.withdrawtolandlord(deps, env, info, token_id, tenant, renting_period, amount, address)
            }

            ExecuteMsg::Withdraw { target, amount } => self.withdraw(deps, info, target, amount),

            ExecuteMsg::SetFeeValue { fee } => self.set_fee_value(deps,info, fee),
            
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

        let longterm_rental = LongTermRental {
            islisted: None,
            price_per_month: 0u64,
            available_period:vec![],
            deposit_amount: Uint128::from(0u64),
            withdrawn_amount: Uint128::from(0u64),
            denom:"ibc/F082B65C88E4B6D5EF1DB243CDA1D331D002759E938A0F5CD3FFDC5D53B3E349".to_string(),
            auto_approve:false,
            cancellation:vec![],
            minimum_stay:0u64,
        };

        let shortterm_rental = ShortTermRental {
            islisted: None,
            price_per_day: 0u64,
            available_period: vec![],
            deposit_amount: Uint128::from(0u64),
            withdrawn_amount: Uint128::from(0u64),
            denom: "ibc/F082B65C88E4B6D5EF1DB243CDA1D331D002759E938A0F5CD3FFDC5D53B3E349".to_string(),
            auto_approve: false,
            cancellation:vec![],
            minimum_stay:0u64,
        };

        let sell = Sell {
            islisted:None,
            auto_approve:false,
            price:0u64,
            denom:"ibc/F082B65C88E4B6D5EF1DB243CDA1D331D002759E938A0F5CD3FFDC5D53B3E349".to_string(),            
        };

        // create the token
        let token = TokenInfo {
            owner: info.sender.clone(),
            approvals: vec![],
            rentals:vec![],
            bids:vec![],
            longterm_rental,
            shortterm_rental,
            sell,
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

    pub fn set_fee_value(&self,deps:DepsMut,info: MessageInfo, fee:u64) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;
        self.set_fee(deps.storage, fee)?;
        Ok(Response::new()
            .add_attribute("action", "setfee"))
    }

    pub fn withdraw(&self, deps: DepsMut, info: MessageInfo, target:String, amount:Coin) -> Result<Response<C>, ContractError> {
        
        cw_ownable::assert_owner(deps.storage, &info.sender)?;


        if amount.amount.clone() > self.get_balance(deps.storage, amount.denom.clone())? {
            return Err(ContractError::UnavailableAmount {});
        }

        self.decrease_balance(deps.storage, amount.denom.clone(), amount.amount.clone())?;
        
        Ok(Response::new()
            .add_attribute("action", "withdraw")
            .add_message(BankMsg::Send {
                    to_address: target,
                    amount: vec![amount],
                })
        )
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
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_send(deps.as_ref(), &env, &info, &token)?;
        // set owner and remove existing approvals
        let prev_owner = token.owner;
        token.owner = deps.api.addr_validate(&recipient)?;
        token.approvals = vec![];
        let fee_percentage = self.get_fee(deps.storage)?;

        let mut position: i32 = -1;
        let mut amount = Uint128::from(0u64);
        for (i, item) in token.bids.iter().enumerate() {
            if item.address == recipient.to_string()
            {
                position = i as i32;
                amount = item.offer.into();
                break;  
            }
        }

        if position != -1 && amount > Uint128::new(0) {
            self.increase_balance(deps.storage, token.sell.denom.clone(), Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000))?;
        }
        let amount_after_fee = amount.checked_sub(Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000)).unwrap_or_default();
        token.bids.retain(|bid| bid.address != recipient);
        self.tokens.save(deps.storage, &token_id, &token)?;
        if amount > Uint128::new(0) {
            Ok(Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("sender", info.sender.clone())
            .add_attribute("token_id", token_id)
            .add_message(BankMsg::Send {
                to_address: prev_owner.to_string(),
                amount: vec![Coin {
                    denom: token.sell.denom,
                    amount: amount_after_fee,
                }],
            }))
        } else {
            Ok(Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("sender", info.sender.clone())
            .add_attribute("token_id", token_id))
        }
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
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, token_id)?;
        // ensure we have permissions
        self.check_can_send(deps.as_ref(), env, info, &token)?;
        // set owner and remove existing approvals
        token.owner = deps.api.addr_validate(recipient)?;
        token.approvals = vec![];

        self.tokens.save(deps.storage, token_id, &token)?;
        Ok(Response::new()
        .add_attribute("action", "_transfer_nft")
        .add_attribute("sender", info.sender.clone())
        .add_attribute("token_id", token_id))
    }

    pub fn setmetadata(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        token_uri: String,
        // expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;
        self.check_can_edit_short(&env, &token)?;
        self.check_can_edit_long(&env, &token)?;
        token.token_uri = Some(token_uri);
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setmetadata")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setextension(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        extension: T,
        // expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;
        self.check_can_edit_short(&env, &token)?;
        self.check_can_edit_long(&env, &token)?;
        token.extension = extension;
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setextension")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setlistforsell(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        islisted:bool,
        token_id: String,
        denom: String,
        price: u64,
        auto_approve: bool,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        token.sell.islisted = Some(islisted);
        token.sell.price = price;
        token.sell.auto_approve = auto_approve;
        token.sell.denom = denom;
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setlistforsell")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setbidtobuy(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;

        let mut position: i32 = -1;
        let mut amount = Uint128::from(0u64);
        for (i, item) in token.bids.iter().enumerate() {
            if item.address == info.sender.to_string()
            {
                position = i as i32;
                amount = item.offer.into();
                break;
            }
        }

        if position == -1 {
            if info.funds[0].denom != token.sell.denom {
                return Err(ContractError::InvalidDeposit {});
            }
            if info.funds[0].amount
                < Uint128::from(token.sell.price)
            {
                return Err(ContractError::InsufficientDeposit {});
            }

            if token.sell.auto_approve {
                // update the approval list (remove any for the same spender before adding)
                let expires = Expiration::Never {  };
                token.approvals.retain(|apr| apr.spender != info.sender);
                let approval = Approval {
                    spender: info.sender.clone(),
                    expires,
                };
                token.approvals.push(approval);
                
            }
            let bid = Bid {
                address: info.sender.to_string(),
                offer:info.funds[0].amount,
            };
            token.bids.push(bid);
        }

        else {
            // update the approval list (remove any for the same spender before adding)
            token.bids.retain(|item| item.address != info.sender);
        }

        self.tokens.save(deps.storage, &token_id, &token)?;
        if position != -1 && (amount > Uint128::from(0u64)) {
            Ok(Response::new()
            .add_attribute("action", "setbidtobuy")
            .add_attribute("sender", info.sender.clone())
            .add_attribute("token_id", token_id)
            .add_message(BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: vec![Coin {
                    denom: token.sell.denom,
                    amount: amount,
                }],
            }))
        }
        else {
            Ok(Response::new()
            .add_attribute("action", "setbidtobuy")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
        }

    }

    pub fn setlistforshorttermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        denom: String,
        price_per_day: u64,
        auto_approve: bool,
        available_period: Vec<String>,
        minimum_stay:u64,
        cancellation:Vec<CancellationItem>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;
        self.check_can_edit_short(&env, &token)?;

        token.shortterm_rental.islisted = Some(true);
        token.shortterm_rental.price_per_day = price_per_day;
        token.shortterm_rental.available_period = available_period;
        token.shortterm_rental.auto_approve = auto_approve;
        token.shortterm_rental.denom = denom;
        token.shortterm_rental.minimum_stay = minimum_stay;
        token.shortterm_rental.cancellation = cancellation;
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setlistforshorttermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setunlistforshorttermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;
        self.check_can_edit_short(&env, &token)?;

        token.shortterm_rental.islisted = None;
        token.shortterm_rental.price_per_day = 0u64;
        token.shortterm_rental.available_period = vec![];
        token.shortterm_rental.auto_approve = false;
        token.shortterm_rental.minimum_stay = 0u64;
        token.shortterm_rental.cancellation = vec![];
        token.shortterm_rental.denom = "".to_string();

        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setunlistforshorttermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setreservationforshortterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        renting_period: Vec<String>,
        guests:usize,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        let new_checkin = renting_period[0].parse::<u64>();
        let new_checkin_timestamp;

        match new_checkin {
            Ok(timestamp) => {
                new_checkin_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }
        let new_checkout = renting_period[1].parse::<u64>();
        let new_checkout_timestamp;

        match new_checkout {
            Ok(timestamp) => {
                new_checkout_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }

        if ((new_checkout_timestamp - new_checkin_timestamp)/ 86400) < token.shortterm_rental.minimum_stay {
            return Err(ContractError::LessThanMinimum {});
        }

        let mut placetoreserve: i32 = -1;
        let lenofrentals = token.rentals.len();

        let mut flag = false;
        for (i, traveler) in token.rentals.iter().enumerate() {
            let checkin = traveler.renting_period[0];
            let checkout = traveler.renting_period[1];
            if new_checkout_timestamp < checkin {
                if i == 0 {
                    placetoreserve = 0;
                    break;
                } else if flag {
                    placetoreserve = i as i32;
                    break;
                }
            } else if checkout < new_checkin_timestamp {
                flag = true;
                if i == lenofrentals - 1 {
                    placetoreserve = lenofrentals as i32;
                    break;
                }
            } else {
                flag = false;
            }
        }

        if placetoreserve == -1 {
            if lenofrentals > 0 {
                return Err(ContractError::UnavailablePeriod {});
            } else {
                placetoreserve = 0;
            }
        }

        if info.funds[0].denom != token.shortterm_rental.denom {
            return Err(ContractError::InvalidDeposit {});
        }
        let sent_amount = info.funds[0].amount;
        let fee_percentage = self.get_fee(deps.storage)?;
        let rent_amount = token.shortterm_rental.price_per_day
        * (new_checkout_timestamp - new_checkin_timestamp)/(86400);
        if sent_amount
            < Uint128::from(rent_amount) + Uint128::new((u128::from(rent_amount) * u128::from(fee_percentage)) / 10000)
        {
            return Err(ContractError::InsufficientDeposit {});
        }

        self.increase_balance(deps.storage, info.funds[0].denom.clone(), sent_amount - Uint128::from(rent_amount))?;

        let traveler = Rental {
            denom:token.shortterm_rental.denom.clone(),
            rental_type:false,
            approved_date:None,
            deposit_amount: Uint128::from(rent_amount),
            renting_period: vec![new_checkin_timestamp, new_checkout_timestamp],
            address: Some(info.sender.clone()),
            approved: token.shortterm_rental.auto_approve,
            cancelled:false,
            guests:guests,
        };

        // token.shortterm_rental.deposit_amount += sent_amount;
        token
            .rentals
            .insert(placetoreserve as usize, traveler);

        self.tokens.save(deps.storage, &token_id, &token)?;

        // if token.shortterm_rental.auto_approve {
        //     Ok(Response::new()
        //         .add_attribute("action", "setreservationforshortterm")
        //         .add_attribute("sender", info.sender)
        //         .add_attribute("token_id", token_id)
        //         .add_message(BankMsg::Send {
        //             to_address: token.owner.into_string(),
        //             amount: vec![Coin {
        //                 denom: token.shortterm_rental.denom,
        //                 amount: sent_amount,
        //             }],
        //         }))
        // } else {
            Ok(Response::new()
                .add_attribute("action", "setreservationforshortterm")
                .add_attribute("sender", info.sender)
                .add_attribute("token_id", token_id))
        // }
    }

    pub fn setapproveforshortterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        token_id: String,
        renting_period: Vec<String>,
        traveler: String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        let current_time = env.block.time.seconds();

        let check_in_time = renting_period[0].parse::<u64>();
        let check_in_time_timestamp;

        match check_in_time {
            Ok(timestamp) => {
                check_in_time_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }


        if check_in_time_timestamp <= current_time {
            return Err(ContractError::RentalAlreadyStarted {});
        }

        let mut position: i32 = -1;
        // let mut amount = Uint128::from(0u64);
        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(Addr::unchecked(traveler.clone()))
                // && item.renting_period == renting_period
                && item.renting_period[0].to_string() == renting_period[0]
                && item.renting_period[1].to_string() == renting_period[1]
                && !item.approved
            {
                position = i as i32;
                // amount = item.deposit_amount;
                break;
            }
        }
        if position == -1 {
            return Err(ContractError::ApprovedAlready {});
        }
        token.rentals[position as usize].approved = true;

        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setreservationforshortterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            // .add_message(BankMsg::Send {
            //     to_address: token.owner.into_string(),
            //     amount: vec![Coin {
            //         denom: token.shortterm_rental.denom,
            //         amount: amount,
            //     }],
            // })
        )
    }

    pub fn rejectreservationforshortterm(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        traveler: String,
        renting_period: Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        // if token.shortterm_rental.auto_approve {
        //     return Err(ContractError::ApprovedAlready {});
        // }
        // let auto_approved = token.shortterm_rental.auto_approve;

        let mut position: i32 = -1;
        let mut refundable_amount:Uint128 = Uint128::new(0);
        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(Addr::unchecked(traveler.clone()))
                // && item.renting_period == renting_period
                && item.renting_period[0].to_string() == renting_period[0]
                && item.renting_period[1].to_string() == renting_period[1]
            {
                position = i as i32;
                if item.approved {
                    // return Err(ContractError::ApprovedAlready {});
                    refundable_amount = item.deposit_amount; 
                } else {
                    refundable_amount = item.deposit_amount;
                    
                }
            }
        }
        if position == -1 {
            return Err(ContractError::NotReserved {});
        } else {
            token.rentals.remove(position as usize);
            self.tokens.save(deps.storage, &token_id, &token)?;
        }

        Ok(Response::new()
            .add_attribute("action", "rejectreservationforshortterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            .add_message(BankMsg::Send {
                to_address: traveler,
                amount: vec![Coin {
                    denom: token.shortterm_rental.denom,
                    amount: refundable_amount,
                }],
            }))
    }



    pub fn cancelreservationafterapprovalforshortterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        token_id: String,
        renting_period: Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // let is_approved = token.shortterm_rental.auto_approve;

        let mut position: i32 = -1;
        let mut amount = Uint128::new(0);
        let traveler_address = info.sender.to_string();
        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(info.sender.clone()) 
            // && item.renting_period == renting_period
            && item.renting_period[0].to_string() == renting_period[0]
                && item.renting_period[1].to_string() == renting_period[1]
             {
                if item.approved && !item.cancelled {
                    position = i as i32;
                    amount = item.deposit_amount;
                } else {
                    return Err(ContractError::NotApproved {});
                }
            }
        }
        let mut refundable_amount = Uint128::new(0);
        let mut cancellation = token.shortterm_rental.cancellation.clone();
        cancellation.sort_by(|a,b| b.percentage.cmp(&a.percentage));
        let current_time = env.block.time.seconds();

        let check_in_time = renting_period[0].parse::<u64>();
        let check_in_time_timestamp;

        match check_in_time {
            Ok(timestamp) => {
                check_in_time_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }


        if check_in_time_timestamp <= current_time {
            return Err(ContractError::RentalAlreadyStarted {});
        }

        let diff_days = (check_in_time_timestamp - current_time)/86400;
        for (_i, item) in cancellation.iter().enumerate() {
            if item.deadline < diff_days {
                refundable_amount =  Uint128::new((amount.u128() * u128::from(item.percentage)) / 100);
                break;
            }
        }

        if cancellation.len() == 0 {
            refundable_amount = amount;
        }


        if position != -1 {
            // token.rentals.remove(position as usize);

            token.rentals[position as usize].cancelled = true;
            token.rentals[position as usize].deposit_amount = amount - refundable_amount;


            self.tokens.save(deps.storage, &token_id, &token)?;
            if refundable_amount > Uint128::new(0) {
                Ok(Response::new()
                .add_attribute("action", "cancelreservationafterapprovalforshortterm")
                .add_attribute("sender", info.sender)
                .add_attribute("token_id", token_id)
                .add_message(BankMsg::Send {
                    to_address: traveler_address,
                    amount: vec![Coin {
                        denom: token.shortterm_rental.denom,
                        amount: refundable_amount,
                    }],
                }))
            }
            else {
            Ok(Response::new()
                .add_attribute("action", "cancelreservationafterapprovalforshortterm")
                .add_attribute("sender", info.sender)
                .add_attribute("token_id", token_id)
                )
            }

        } else {
            return Err(ContractError::NotReserved {});
        }
    }



    pub fn cancelreservationbeforeapprovalforshortterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        renting_period: Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // let is_approved = token.shortterm_rental.auto_approve;

        // if is_approved {
        //     return Err(ContractError::ApprovedAlready {});
        // }
        let mut position: i32 = -1;
        let mut amount = Uint128::from(0u64);
        let traveler_address = info.sender.to_string();
        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(info.sender.clone()) 
            // && item.renting_period == renting_period
            && item.renting_period[0].to_string() == renting_period[0]
                && item.renting_period[1].to_string() == renting_period[1]
             {
                if item.approved {
                    return Err(ContractError::ApprovedAlready {});
                } else {
                    position = i as i32;
                    amount = item.deposit_amount;
                }
            }
        }

        if position != -1 {
            token.rentals.remove(position as usize);
            self.tokens.save(deps.storage, &token_id, &token)?;
            Ok(Response::new()
                .add_attribute("action", "cancelreservationbeforeapprovalforshortterm")
                .add_attribute("sender", info.sender)
                .add_attribute("token_id", token_id)
                .add_message(BankMsg::Send {
                    to_address: traveler_address,
                    amount: vec![Coin {
                        denom: token.shortterm_rental.denom,
                        amount: amount,
                    }],
                }))
        } else {
            return Err(ContractError::NotReserved {});
        }
    }

    pub fn finalizeshorttermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        traveler: String,
        renting_period: Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;

        self.check_can_send(deps.as_ref(), &env, &info, &token)?;

        let mut position: i32 = -1;
        let mut amount = Uint128::from(0u64);

        let current_time = env.block.time.seconds();

        let check_out_time = renting_period[1].parse::<u64>();
        let check_out_time_timestamp;

        match check_out_time {
            Ok(timestamp) => {
                check_out_time_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }



        let mut target = "".to_string();

        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(Addr::unchecked(traveler.clone()))
                // && item.renting_period == renting_period
                && item.renting_period[0].to_string() == renting_period[0]
                && item.renting_period[1].to_string() == renting_period[1]
            {
                position = i as i32;
                amount = item.deposit_amount;

                if item.cancelled {
                    target = token.owner.to_string();
                }
                if !item.cancelled {
                    if !item.approved {
                        target = traveler.clone();
                        // return  Err(ContractError::NotApproved {});
                    }
                    if item.approved {
                        target = token.owner.to_string();
                        let fee_percentage = self.get_fee(deps.storage)?;
                        self.increase_balance(deps.storage, token.shortterm_rental.denom.clone(), Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000))?;
                        amount -= Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000);
                    }
                }

            }
        }
        if position == -1 {
            return Err(ContractError::NotReserved {});
        } else {

            if check_out_time_timestamp > current_time 
            && !token.rentals[position as usize].cancelled
             {
                    return Err(ContractError::RentalActive {});              
            }

            
            token.rentals.remove(position as usize);
            self.tokens.save(deps.storage, &token_id, &token)?;
        }



        if amount > Uint128::new(0) {
        Ok(Response::new()
            .add_attribute("action", "finalizeshorttermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            .add_message(BankMsg::Send {
                to_address: target.clone(),
                amount: vec![Coin {
                    denom: token.shortterm_rental.denom,
                    amount: amount,
                }],
            }))            
        } 
        else {
            Ok(Response::new()
            .add_attribute("action", "finalizeshorttermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            )
        }

    }

    pub fn setlistforlongtermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        denom: String,
        price_per_month: u64,
        auto_approve: bool,
        available_period: Vec<String>,
        minimum_stay: u64,
        cancellation: Vec<CancellationItem>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;
        self.check_can_edit_long(&env, &token)?;

        token.longterm_rental.islisted = Some(true);
        token.longterm_rental.price_per_month = price_per_month;
        token.longterm_rental.available_period = available_period;
        token.longterm_rental.auto_approve = auto_approve;
        token.longterm_rental.denom = denom;
        token.longterm_rental.minimum_stay = minimum_stay;
        token.longterm_rental.cancellation = cancellation;
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
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;
        self.check_can_edit_long(&env, &token)?;

        token.longterm_rental.islisted = None;
        token.longterm_rental.price_per_month = 0u64;
        token.longterm_rental.available_period = vec![];
        token.longterm_rental.auto_approve = false;
        token.longterm_rental.minimum_stay = 0u64;
        token.longterm_rental.cancellation = vec![];
        token.longterm_rental.denom = "".to_string();

        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setunlistforlongtermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setreservationforlongterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        renting_period: Vec<String>,
        guests:usize,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        let new_checkin = renting_period[0].parse::<u64>();
        let new_checkin_timestamp;

        match new_checkin {
            Ok(timestamp) => {
                new_checkin_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }
        let new_checkout = renting_period[1].parse::<u64>();
        let new_checkout_timestamp;

        match new_checkout {
            Ok(timestamp) => {
                new_checkout_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }

        if ((new_checkout_timestamp - new_checkin_timestamp)/ 86400) < token.longterm_rental.minimum_stay {
            return Err(ContractError::LessThanMinimum {});
        }

        let mut placetoreserve: i32 = -1;
        let lenofrentals = token.rentals.len();

        let mut flag = false;
        for (i, tenant) in token.rentals.iter().enumerate() {
            let checkin = tenant.renting_period[0];
            let checkout = tenant.renting_period[1];
            if new_checkout_timestamp < checkin {
                if i == 0 {
                    placetoreserve = 0;
                    break;
                } else if flag {
                    placetoreserve = i as i32;
                    break;
                }
            } else if checkout < new_checkin_timestamp {
                flag = true;
                if i == lenofrentals - 1 {
                    placetoreserve = lenofrentals as i32;
                    break;
                }
            } else {
                flag = false;
            }
        }

        if placetoreserve == -1 {
            if lenofrentals > 0 {
                return Err(ContractError::UnavailablePeriod {});
            } else {
                placetoreserve = 0;
            }
        }

        let tenant = Rental {
            denom:token.longterm_rental.denom.clone(),
            rental_type:true,
            approved:token.longterm_rental.auto_approve,
            deposit_amount: Uint128::from(0u64),
            renting_period: vec![new_checkin_timestamp, new_checkout_timestamp],
            address: Some(info.sender.clone()),
            approved_date: None,
            cancelled:false,
            guests,
        };

        token
            .rentals
            .insert(placetoreserve as usize, tenant);

        self.tokens.save(deps.storage, &token_id, &token)?;
            Ok(Response::new()
                .add_attribute("action", "setreservationforlongterm")
                .add_attribute("sender", info.sender)
                .add_attribute("token_id", token_id))
    }

    pub fn cancelreservationbeforeapprovalforlongterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        renting_period: Vec<String>
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;

        let mut position: i32 = -1;
        let mut amount = Uint128::from(0u64);
        let tenant_address = info.sender.to_string();
        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(info.sender.clone()) && item.renting_period[0].to_string() == renting_period[0]
            && item.renting_period[1].to_string() == renting_period[1]
             {
                if item.approved_date.is_some() {
                    return Err(ContractError::ApprovedAlready {});
                } else {
                    position = i as i32;
                    amount = item.deposit_amount;
                }
            }
        }

        if position == -1 {
            return Err(ContractError::NotReserved {});
        }
        else {
            token.rentals.remove(position as usize);
            self.tokens.save(deps.storage, &token_id, &token)?;
        }


        if amount > Uint128::new(0) {
            Ok(Response::new()
            .add_attribute("action", "cancelreservationbeforeapprovalforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            .add_message(BankMsg::Send {
                to_address: tenant_address,
                amount: vec![Coin {
                    denom: token.longterm_rental.denom,
                    amount: amount,
                }],
            }))
        }
        else {
            Ok(Response::new()
            .add_attribute("action", "cancelreservationbeforeapprovalforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
        } 
    }


    pub fn rejectreservationforlongterm(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        tenant: String,
        renting_period: Vec<String>
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        let mut position: i32 = -1;
        let mut refundable_amount:Uint128 = Uint128::new(0);
        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(Addr::unchecked(tenant.clone())) && item.renting_period[0].to_string() == renting_period[0]
            && item.renting_period[1].to_string() == renting_period[1]
                {
                    refundable_amount = item.deposit_amount;
                    position = i as i32;
                }
        }
        if position == -1 {
            return Err(ContractError::NotReserved {});
        } else {
            token.rentals.remove(position as usize);
            self.tokens.save(deps.storage, &token_id, &token)?;
        }

        if refundable_amount > Uint128::new(0) {
            Ok(Response::new()
            .add_attribute("action", "rejectreservationforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            .add_message(BankMsg::Send {
                to_address: tenant,
                amount: vec![Coin {
                    denom: token.longterm_rental.denom,
                    amount: refundable_amount,
                }],
            }))
        }
        else {
            Ok(Response::new()
            .add_attribute("action", "rejectreservationforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            )
        }

        
    }


    pub fn depositforlongtermrental(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        renting_period: Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;

        let mut position: i32 = -1;
        let sent_amount = info.funds[0].amount;
        if info.funds[0].denom != token.longterm_rental.denom {
            return Err(ContractError::InvalidDeposit {});
        }

        if sent_amount > Uint128::new(0)
        {
            for (i, item) in token.rentals.iter().enumerate() {
                if item.address == Some(info.sender.clone()) && item.renting_period[0].to_string() == renting_period[0]
                && item.renting_period[1].to_string() == renting_period[1]
                {
                   position = i as i32;
                }
            }
    
            if position == -1 {
                return Err(ContractError::NotReserved {});
            }
    
            // let fee_percentage = self.get_fee(deps.storage)?;
    
            token.rentals[position as usize].deposit_amount += 
            Uint128::from(sent_amount);
            // self.increase_balance(deps.storage, info.funds[0].denom.clone(), Uint128::new((u128::from(sent_amount) * u128::from(fee_percentage)) / 10000))?;
            self.tokens.save(deps.storage, &token_id, &token)?;
        }
        else {
            return Err(ContractError::InsufficientDeposit {});
        }



        Ok(Response::new()
            .add_attribute("action", "depositforlongtermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn setapproveforlongterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        token_id: String,
        renting_period: Vec<String>,
        tenant: String,
        approved_date:String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;
        let current_time = env.block.time.seconds();

        let check_in_time = renting_period[0].parse::<u64>();
        let check_in_time_timestamp;

        match check_in_time {
            Ok(timestamp) => {
                check_in_time_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }


        if check_in_time_timestamp <= current_time {
            return Err(ContractError::RentalAlreadyStarted {});
        }

        let mut position: i32 = -1;
        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(Addr::unchecked(tenant.clone()))
                && item.renting_period[0].to_string() == renting_period[0]
                && item.renting_period[1].to_string() == renting_period[1]
            {
                position = i as i32;
                break;
            }
        }
        if position == -1 {
            return Err(ContractError::NotReserved {});
        }
        token.rentals[position as usize].approved_date = Some(approved_date);

        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setapproveforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
        )
    }

    pub fn cancelreservationafterapprovalforlongterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        renting_period: Vec<String>
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;

        let mut position: i32 = -1;
        // let mut amount = Uint128::from(0u64);
        // let tenant_address = info.sender.to_string();
        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(info.sender.clone()) && item.renting_period[0].to_string() == renting_period[0]
            && item.renting_period[1].to_string() == renting_period[1]
             {
                if item.approved_date.is_none() {
                    return Err(ContractError::NotApproved {});
                } else {
                    position = i as i32;
                    // amount = item.deposit_amount;
                }
            }
        }

        if position != -1 {
            // token.rentals.remove(position as usize);
            token.rentals[position as usize].cancelled = true;
            self.tokens.save(deps.storage, &token_id, &token)?;
            Ok(Response::new()
            .add_attribute("action", "cancelreservationafterapprovalforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
        } else {
            return Err(ContractError::NotReserved {});
        }
    }

    pub fn finalizelongtermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        tenant: String,
        renting_period: Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;

        self.check_can_send(deps.as_ref(), &env, &info, &token)?;

        let mut position: i32 = -1;
        let mut amount = Uint128::from(0u64);

        let current_time = env.block.time.seconds();

        let check_out_time = renting_period[1].parse::<u64>();
        let check_out_time_timestamp;

        match check_out_time {
            Ok(timestamp) => {
                check_out_time_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }



        let mut target = "".to_string();

        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(Addr::unchecked(tenant.clone()))
                && item.renting_period[0].to_string() == renting_period[0]
                && item.renting_period[1].to_string() == renting_period[1]
            {
                position = i as i32;
                amount = item.deposit_amount;

                if item.cancelled {
                    target = token.owner.to_string();
                    let fee_percentage = self.get_fee(deps.storage)?;
                    self.increase_balance(deps.storage, token.longterm_rental.denom.clone(), Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000))?;
                    amount -= Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000);
                }
                if !item.cancelled {
                    if item.approved_date.is_none() {
                        target = tenant.clone();
                        // return  Err(ContractError::NotApproved {});
                    }
                    else {
                        target = token.owner.to_string();
                        let fee_percentage = self.get_fee(deps.storage)?;
                        self.increase_balance(deps.storage, token.longterm_rental.denom.clone(), Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000))?;
                        amount -= Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000);
                    }
                }

            }
        }
        if position == -1 {
            return Err(ContractError::NotReserved {});
        } else {

            if check_out_time_timestamp > current_time 
            && !token.rentals[position as usize].cancelled
             {
                    return Err(ContractError::RentalActive {});              
            }

            
            token.rentals.remove(position as usize);
            self.tokens.save(deps.storage, &token_id, &token)?;
        }

        if amount > Uint128::new(0) {
            Ok(Response::new()
                .add_attribute("action", "finalizelongtermrental")
                .add_attribute("sender", info.sender)
                .add_attribute("token_id", token_id)
                .add_message(BankMsg::Send {
                    to_address: target.clone(),
                    amount: vec![Coin {
                        denom: token.longterm_rental.denom,
                        amount: amount,
                    }],
                }))            
        } 
        else {
            Ok(Response::new()
            .add_attribute("action", "finalizelongtermrental")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            )
        }

    }

    pub fn withdrawtolandlord(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        tenant: String,
        renting_period: Vec<String>,
        amount:u64,
        address:String
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;

        self.check_can_send(deps.as_ref(), &env, &info, &token)?;

        let mut position = -1;

        let current_time = env.block.time.seconds();

        let check_in_time = renting_period[0].parse::<u64>();
        let check_in_time_timestamp;

        match check_in_time {
            Ok(timestamp) => {
                check_in_time_timestamp = timestamp;
            }
            Err(_e) => {
                return Err(ContractError::NotReserved {});
            }
        }

        if current_time <= check_in_time_timestamp {
            return Err(ContractError::RentalNotActivated {});
        }
        let fee_percentage = self.get_fee(deps.storage)?;

        for (i, item) in token.rentals.iter().enumerate() {
            if item.address == Some(Addr::unchecked(tenant.clone()))
                && item.renting_period[0].to_string() == renting_period[0]
                && item.renting_period[1].to_string() == renting_period[1]
            {
                position = i as i32;
                if item.cancelled {
                    return Err(ContractError::NotApproved { });
                }
                if item.deposit_amount - Uint128::from(token.longterm_rental.price_per_month) < Uint128::from(amount)  {
                    return Err(ContractError::UnavailableAmount {  });
                }
            }
        }
        if position == -1 {
            return Err(ContractError::NotReserved {});
        } else {
            self.increase_balance(deps.storage, token.longterm_rental.denom.clone(), Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000))?;
            token.rentals[position as usize].deposit_amount -= Uint128::from(amount);
            self.tokens.save(deps.storage, &token_id, &token)?;
        }
        Ok(Response::new()
            .add_attribute("action", "withdrawtolandlord")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            .add_message(BankMsg::Send {
                to_address: address,
                amount: vec![Coin {
                    denom: token.longterm_rental.denom,
                    amount: Uint128::from(amount) - Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000),
                }],
            }))            
    }

    // pub fn withdrawtolandlord(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     token_id: String,
    //     amount: Uint128,
    //     denom: String,
    // ) -> Result<Response<C>, ContractError> {
    //     let mut token = self.tokens.load(deps.storage, &token_id)?;
    //     self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

    //     if token.longterm_rental.deposit_amount < token.longterm_rental.withdrawn_amount + amount {
    //         return Err(ContractError::UnavailableAmount {});
    //     }

    //     if !token.longterm_rental.ejari_flag.is_some() {
    //         return Err(ContractError::EjariNotConfirmed {});
    //     }

    //     token.longterm_rental.withdrawn_amount += amount;
    //     self.tokens.save(deps.storage, &token_id, &token)?;
    //     Ok(Response::new()
    //         .add_attribute("action", "withdrawtolandlord")
    //         .add_message(BankMsg::Send {
    //             to_address: token.owner.into_string(),
    //             amount: vec![Coin { denom, amount }],
    //         }))
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

    pub fn check_can_edit_short(
        &self,
        env:&Env,
        token:&TokenInfo<T>,
    ) -> Result<(), ContractError> {
        if token.rentals.len() == 0 {
            return Ok(());
        }
        else {
            // token.rentals[token.rentals.len()-1].renting_period[1]
            let current_time = env.block.time.seconds();
            let last_check_out_time = token.rentals[token.rentals.len()-1].renting_period[1];
            if last_check_out_time < current_time {
                return Ok(());
            }
            else {
                return Err(ContractError::RentalActive {});
            }
        }
    }

    pub fn check_can_edit_long(
        &self,
        env:&Env,
        token:&TokenInfo<T>,
    ) -> Result<(), ContractError> {
        if token.rentals.len() == 0 {
            return Ok(());
        }
        else {
            let current_time = env.block.time.seconds();
            let last_check_out_time = token.rentals[token.rentals.len()-1].renting_period[1];
            if last_check_out_time < current_time {
                return Ok(());
            }
            else {
                return Err(ContractError::RentalActive {});
            }
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
