// use chrono::NaiveDate;
use cw_ownable::OwnershipError;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{
    Addr, BankMsg, Binary, Coin, CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};

use cw721::{
    CancellationItem, ContractInfoResponse, Cw721Execute, Cw721ReceiveMsg, Expiration, Landlord, LongTermRental, ShortTermRental, Tenant, Traveler
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
            // ExecuteMsg::CancelApproveForShortterm {
            //     token_id,
            //     traveler,
            //     renting_period,
            // } => self.rejectreservationforshortterm(
            //     deps,
            //     env,
            //     info,
            //     token_id,
            //     traveler,
            //     renting_period,
            // ),

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


            // ExecuteMsg::SetUnlistForLongtermRental { token_id } => {
            //     self.setunlistforlongtermrental(deps, env, info, token_id)
            // }
            // ExecuteMsg::SetReservationForLongTerm {
            //     token_id,
            //     isreserved,
            //     deposit_amount,
            //     deposit_denom,
            //     renting_period,
            // } => self.setreservationforlongterm(
            //     deps,
            //     info,
            //     token_id,
            //     isreserved,
            //     deposit_amount,
            //     deposit_denom,
            //     renting_period,
            // ),
            // ExecuteMsg::SetListForLongTermRental {
            //     token_id,
            //     islisted,
            //     denom,
            //     price_per_month,
            //     refundable_deposit,
            //     available_period,
            // } => self.setlistforlongtermrental(
            //     deps,
            //     env,
            //     info,
            //     token_id,
            //     islisted,
            //     denom,
            //     price_per_month,
            //     refundable_deposit,
            //     available_period,
            // ),
            // ExecuteMsg::RejectReservationForLongterm { token_id } => {
            //     self.rejectreservationforlongterm(deps, env, info, token_id)
            // }
            // ExecuteMsg::CancelReservationForLongterm { token_id } => {
            //     self.cancelreservationforlongterm(deps, info, token_id)
            // }
            // ExecuteMsg::ProceedLongtermRental { token_id } => {
            //     self.proceedlongtermrental(deps, env, info, token_id)
            // }
            // ExecuteMsg::SetEjariForLongTermRental { token_id, ejari } => {
            //     self.setejariforlongtermrental(deps, env, info, token_id, ejari)
            // }
            // ExecuteMsg::DepositForLongTermRental { token_id } => {
            //     self.depositforlongtermrental(deps, env, info, token_id)
            // }
            // ExecuteMsg::WithdrawToLandlord {
            //     token_id,
            //     amount,
            //     denom,
            // } => self.withdrawtolandlord(deps, env, info, token_id, amount, denom),
            // ExecuteMsg::FinalizeLongTermRental { token_id } => {
            //     self.finalizelongtermrental(deps, env, info, token_id)
            // }
            
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
            isreserved: None,
            landlord: None,
            tenant: None,
            tenant_address: None,
            deposit_amount: Uint128::from(0u64),
            withdrawn_amount: Uint128::from(0u64),
            renting_flag: None,
            ejari_flag: None,
        };

        let shortterm_rental = ShortTermRental {
            islisted: None,
            travelers: vec![],
            price_per_day: 0u64,
            available_period: vec![],
            deposit_amount: Uint128::from(0u64),
            withdrawn_amount: Uint128::from(0u64),
            denom: "ibc/F082B65C88E4B6D5EF1DB243CDA1D331D002759E938A0F5CD3FFDC5D53B3E349".to_string(),
            auto_approve: false,
            cancellation:vec![],
            minimum_stay:0u64,
        };

        // create the token
        let token = TokenInfo {
            // owner: deps.api.addr_validate(&owner)?,
            owner: info.sender.clone(),
            approvals: vec![],
            longterm_rental: longterm_rental,
            shortterm_rental: shortterm_rental,
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
        self.check_can_edit(&env, &token)?;
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
        self.check_can_edit(&env, &token)?;
        token.extension = extension;
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "setextension")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
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
        self.check_can_edit(&env, &token)?;

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
        self.check_can_edit(&env, &token)?;

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
        // let new_checkin = NaiveDate::parse_from_str(&renting_period[0], "%Y/%m/%d").unwrap();


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
        let lenoftravelers = token.shortterm_rental.travelers.len();

        let mut flag = false;
        for (i, traveler) in token.shortterm_rental.travelers.iter().enumerate() {
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
                if i == lenoftravelers - 1 {
                    placetoreserve = lenoftravelers as i32;
                    break;
                }
            } else {
                flag = false;
            }
        }

        if placetoreserve == -1 {
            if lenoftravelers > 0 {
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

        let traveler = Traveler {
            deposit_amount: Uint128::from(rent_amount),
            renting_period: vec![new_checkin_timestamp, new_checkout_timestamp],
            address: Some(info.sender.clone()),
            approved: token.shortterm_rental.auto_approve,
            cancelled:false,
            guests:guests,
        };

        // token.shortterm_rental.deposit_amount += sent_amount;
        token
            .shortterm_rental
            .travelers
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

        // let is_approved = token.shortterm_rental.auto_approve;

        // if is_approved {
        //     return Err(ContractError::ApprovedAlready {});
        // }

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
        for (i, item) in token.shortterm_rental.travelers.iter().enumerate() {
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
        token.shortterm_rental.travelers[position as usize].approved = true;

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
        for (i, item) in token.shortterm_rental.travelers.iter().enumerate() {
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
            token.shortterm_rental.travelers.remove(position as usize);
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
        for (i, item) in token.shortterm_rental.travelers.iter().enumerate() {
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
            // token.shortterm_rental.travelers.remove(position as usize);

            token.shortterm_rental.travelers[position as usize].cancelled = true;
            token.shortterm_rental.travelers[position as usize].deposit_amount = amount - refundable_amount;


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
        for (i, item) in token.shortterm_rental.travelers.iter().enumerate() {
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
            token.shortterm_rental.travelers.remove(position as usize);
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

        for (i, item) in token.shortterm_rental.travelers.iter().enumerate() {
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
            && !token.shortterm_rental.travelers[position as usize].cancelled
             {
                    return Err(ContractError::RentalActive {});              
            }

            
            token.shortterm_rental.travelers.remove(position as usize);
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
        islisted: bool,
        denom: String,
        price_per_month: u64,
        refundable_deposit: u64,
        available_period: Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        let landlord = Landlord {
            denom: denom,
            price_per_month: price_per_month,
            refundable_deposit: refundable_deposit,
            available_period: available_period,
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
        token_id: String,
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
        token_id: String,
        _isreserved: bool,
        deposit_amount: u64,
        deposit_denom: String,
        renting_period: Vec<String>,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        // let sent_amount = info.funds[0].amount;
        let tenant = Tenant {
            deposit_amount: deposit_amount,
            deposit_denom: deposit_denom,
            renting_period: renting_period,
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

    pub fn rejectreservationforlongterm(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        let mut token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

        token.longterm_rental.isreserved = None;
        token.longterm_rental.tenant_address = None;
        token.longterm_rental.tenant = None;
        self.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "rejectreservationforlongterm")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn cancelreservationforlongterm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
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
        token_id: String,
        ejari: bool,
    ) -> Result<Response<C>, ContractError> {
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
        token_id: String,
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
        env: Env,
        info: MessageInfo,
        token_id: String,
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
        token_id: String,
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
                amount: vec![Coin { denom, amount }],
            }))
    }

    pub fn finalizelongtermrental(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
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

    pub fn check_can_edit(
        &self,
        // _deps:Deps,
        env:&Env,
        // _info:&MessageInfo,
        token:&TokenInfo<T>,
    ) -> Result<(), ContractError> {
        if token.shortterm_rental.travelers.len() == 0 {
            return Ok(());
        }
        else {
            // token.shortterm_rental.travelers[token.shortterm_rental.travelers.len()-1].renting_period[1]
            let current_time = env.block.time.seconds();
            let last_check_out_time = token.shortterm_rental.travelers[token.shortterm_rental.travelers.len()-1].renting_period[1];
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
