use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, BidsResponse, ContractInfoResponse,
    LongTermRental, NftInfoResponse, NumTokensResponse, OperatorResponse, OperatorsResponse,
    OwnerOfResponse, RentalsResponse, Sell, ShortTermRental, TokensResponse,
};
use cosmwasm_std::{CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_utils::Expiration;

pub trait Cw721<T, C>: Cw721Execute<T, C> + Cw721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

pub trait Cw721Execute<T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    type Err: ToString;

    fn transfer_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;

    // fn send_nft(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     contract: String,
    //     token_id: String,
    //     msg: Binary,
    // ) -> Result<Response<C>, Self::Err>;

    fn approve(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, Self::Err>;

    fn revoke(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, Self::Err>;

    fn revoke_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response<C>, Self::Err>;

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;
}

pub trait Cw721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    // TODO: use custom error?
    // How to handle the two derived error types?

    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse>;

    fn num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse>;
    // fn get_fee_value(&self, deps: Deps) -> StdResult<FeeValueResponse>;

    fn nft_info(&self, deps: Deps, token_id: String) -> StdResult<NftInfoResponse<T>>;

    fn nft_longtermrental_info(&self, deps: Deps, token_id: String) -> StdResult<LongTermRental>;
    fn nft_shorttermrental_info(&self, deps: Deps, token_id: String) -> StdResult<ShortTermRental>;
    fn nft_sell_info(&self, deps: Deps, token_id: String) -> StdResult<Sell>;
    fn nft_rentals_info(&self, deps: Deps, token_id: String) -> StdResult<RentalsResponse>;
    fn nft_bids_info(&self, deps: Deps, token_id: String) -> StdResult<BidsResponse>;

    fn owner_of(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse>;

    fn operator(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        operator: String,
        include_expired: bool,
    ) -> StdResult<OperatorResponse>;

    fn operators(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse>;

    fn approval(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        spender: String,
        include_expired: bool,
    ) -> StdResult<ApprovalResponse>;

    fn approvals(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<ApprovalsResponse>;

    fn tokens(
        &self,
        deps: Deps,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_tokens(
        &self,
        deps: Deps,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_nft_info(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<T>>;
}
