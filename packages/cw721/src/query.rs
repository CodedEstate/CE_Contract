use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_utils::Expiration;
#[cw_serde]
pub enum Cw721QueryMsg {
    /// Return the owner of the given token, error if token does not exist
    /// Return type: OwnerOfResponse
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    /// Return type: `ApprovalResponse`
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    /// Return approvals that a token has
    /// Return type: `ApprovalsResponse`
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Return approval of a given operator for all tokens of an owner, error if not set
    /// Return type: `OperatorResponse`
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    /// Return type: `OperatorsResponse`
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    NumTokens {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract: `ContractInfoResponse`
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract: `NftInfoResponse`
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients: `AllNftInfo`
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    /// Return type: TokensResponse.
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    /// Return type: TokensResponse.
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct OwnerOfResponse {
    /// Owner of the token
    pub owner: String,
    /// If set this address is approved to transfer/send the token as well
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: String,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

#[cw_serde]
pub struct Bid {
    /// Account that can transfer/send the token
    pub buyer: String,
    /// price offer
    pub offer: u64,
}

#[cw_serde]
pub struct Landlord {
    pub denom: String,
    pub price_per_month: u64,
    pub refundable_deposit: u64,
    pub available_period: Vec<String>,
}

#[cw_serde]
pub struct Host {
    pub price_per_day: u64,
    // pub refundable_deposit: u64,
    pub available_period: Vec<String>,
}

#[cw_serde]
pub struct Tenant {
    pub deposit_amount: u64,
    pub deposit_denom: String,
    pub renting_period: Vec<String>,
}

#[cw_serde]
pub struct Traveler {
    pub deposit_amount: Uint128,
    pub approved: bool,
    pub cancelled: bool,
    pub renting_period: Vec<u64>,
    pub address: Option<Addr>,
    pub guests: usize,
}

#[cw_serde]
pub struct LongTermRental {
    pub islisted: Option<bool>,
    pub isreserved: Option<bool>,
    pub landlord: Option<Landlord>,
    pub tenant: Option<Tenant>,
    pub tenant_address: Option<Addr>,
    pub deposit_amount: Uint128,
    pub withdrawn_amount: Uint128,
    pub renting_flag: Option<bool>,
    pub ejari_flag: Option<bool>,
}

#[cw_serde]
pub struct CancellationItem {
    pub deadline: u64,
    pub percentage: u64,
}

#[cw_serde]
pub struct ShortTermRental {
    pub islisted: Option<bool>,
    pub auto_approve: bool,
    pub price_per_day: u64,
    pub available_period: Vec<String>,
    pub travelers: Vec<Traveler>,
    pub denom: String,
    pub deposit_amount: Uint128,
    pub withdrawn_amount: Uint128,
    pub cancellation: Vec<CancellationItem>,
    pub minimum_stay: u64,
}

#[cw_serde]
pub struct ApprovalResponse {
    pub approval: Approval,
}

#[cw_serde]
pub struct ApprovalsResponse {
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct OperatorResponse {
    pub approval: Approval,
}

#[cw_serde]
pub struct OperatorsResponse {
    pub operators: Vec<Approval>,
}

#[cw_serde]
pub struct NumTokensResponse {
    pub count: u64,
}

#[cw_serde]
pub struct FeeValueResponse {
    pub fee: u64,
}

#[cw_serde]
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
}

#[cw_serde]
pub struct NftInfoResponse<T> {
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,

    /// You can add any custom metadata here when you extend codedestate
    pub extension: T,
}

#[cw_serde]
pub struct AuctionInfoResponse {
    pub islisted: bool,
    /// Token price for auction
    pub price: u64,
    /// bids of buyers for this token
    pub bids: Vec<Bid>,
}

#[cw_serde]
pub struct AllNftInfoResponse<T> {
    /// Who can transfer the token
    pub access: OwnerOfResponse,
    /// Data on the token itself,
    pub info: NftInfoResponse<T>,
    // pub auction: AuctionInfoResponse,
    // pub mode:Option<String>,
}

#[cw_serde]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_after` in future queries
    /// to achieve pagination.
    pub tokens: Vec<String>,
}
