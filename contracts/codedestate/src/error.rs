use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error(transparent)]
    Version(#[from] cw2::VersionError),

    #[error("token_id already claimed")]
    Claimed {},

    #[error("cannot withdraw such amount")]
    UnavailableAmount {},

    #[error("ejari not verified")]
    EjariNotConfirmed {},

    #[error("not reservated")]
    NotReserved {},

    #[error("Rental is not activated yet")]
    RentalNotActivated {},


    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },
}
