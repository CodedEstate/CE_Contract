use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;
use cosmwasm_std::Coin;
use cw721::CancellationItem;
use cw721::Expiration;
use cw_ownable::{cw_ownable_execute, cw_ownable_query};
use schemars::JsonSchema;

#[cw_serde]
pub struct InstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: String,
}

/// This is like Cw721ExecuteMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg<T, E> {
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft {
        recipient: String,
        token_id: String,
    },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke {
        spender: String,
        token_id: String,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll {
        operator: String,
    },

    Withdraw {
        target: String,
        amount: Coin,
    },
    /// Mint a new NFT, can only be called by the contract minter
    Mint {
        /// Unique ID of the NFT
        token_id: String,
        /// The owner of the newly minter NFT
        owner: String,
        /// Universal resource identifier for this NFT
        /// Should point to a JSON file that conforms to the ERC721
        /// Metadata JSON Schema
        token_uri: Option<String>,
        /// Any custom extension used by this contract
        extension: T,
    },

    SetExtension {
        token_id: String,
        extension: T,
    },

    SetFeeValue {
        fee: u64,
    },

    SetMetadata {
        token_id: String,
        token_uri: String,
    },

    // Short term rental
    SetListForShortTermRental {
        token_id: String,
        denom: String,
        price_per_day: u64,
        auto_approve: bool,
        available_period: Vec<String>,
        minimum_stay: u64,
        cancellation: Vec<CancellationItem>,
    },
    SetUnlistForShorttermRental {
        token_id: String,
    },
    SetReservationForShortTerm {
        token_id: String,
        renting_period: Vec<String>,
        guests: usize,
    },
    CancelReservationForShortterm {
        token_id: String,
        renting_period: Vec<String>,
    },

    CancelRentalForShortterm {
        token_id: String,
        renting_period: Vec<String>,
    },

    // CancelApproveForShortterm {
    //     token_id: String,
    //     traveler: String,
    //     renting_period: Vec<String>,
    // },
    RejectReservationForShortterm {
        token_id: String,
        traveler: String,
        renting_period: Vec<String>,
    },
    SetApproveForShortTerm {
        token_id: String,
        traveler: String,
        renting_period: Vec<String>,
    },
    FinalizeShortTermRental {
        token_id: String,
        traveler: String,
        renting_period: Vec<String>,
    },

    //Long term rental
    // SetListForLongTermRental {
    //     token_id: String,
    //     islisted: bool,
    //     denom: String,
    //     price_per_month: u64,
    //     refundable_deposit: u64,
    //     available_period: Vec<String>,
    // },

    // SetUnlistForLongtermRental {
    //     token_id: String,
    // },

    // RejectReservationForLongterm {
    //     token_id: String,
    // },

    // CancelReservationForLongterm {
    //     token_id: String,
    // },

    // ProceedLongtermRental {
    //     token_id: String,
    // },

    // SetReservationForLongTerm {
    //     token_id: String,
    //     isreserved: bool,
    //     deposit_amount: u64,
    //     deposit_denom: String,
    //     renting_period: Vec<String>,
    // },

    // SetEjariForLongTermRental {
    //     token_id: String,
    //     ejari: bool,
    // },

    // DepositForLongTermRental {
    //     token_id: String,
    // },

    // WithdrawToLandlord {
    //     token_id: String,
    //     amount: Uint128,
    //     denom: String,
    // },

    // FinalizeLongTermRental {
    //     token_id: String,
    // },
    /// Burn an NFT the sender has access to
    Burn {
        token_id: String,
    },

    /// Extension msg
    Extension {
        msg: E,
    },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg<Q: JsonSchema> {
    /// Return the owner of the given token, error if token does not exist
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    #[returns(cw721::ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    /// Return approvals that a token has
    #[returns(cw721::ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Return approval of a given operator for all tokens of an owner, error if not set
    #[returns(cw721::OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    #[returns(cw721::OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(cw721::NumTokensResponse)]
    NumTokens {},

    #[returns(cw721::FeeValueResponse)]
    GetFee {},

    #[returns(u64)]
    GetBalance { denom: String },
    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(cw721::NftInfoResponse<Q>)]
    NftInfo { token_id: String },

    #[returns(cw721::LongTermRental)]
    NftInfoLongTermRental { token_id: String },

    #[returns(cw721::ShortTermRental)]
    NftInfoShortTermRental { token_id: String },

    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw721::AllNftInfoResponse<Q>)]
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::TokensResponse)]
    AllTokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Return the minter
    #[returns(MinterResponse)]
    Minter {},

    /// Extension query
    #[returns(())]
    Extension { msg: Q },
}

/// Shows who can mint these tokens
#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}
