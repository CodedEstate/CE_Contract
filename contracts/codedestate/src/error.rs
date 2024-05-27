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

    #[error("Insufficient deposit amount")]
    InsufficientDeposit {},

    #[error("ejari not verified")]
    EjariNotConfirmed {},

    #[error("not reservated")]
    NotReserved {},

    #[error("Approved already")]
    ApprovedAlready {},

    #[error("Invalid deposit denom")]
    InvalidDeposit {},

    #[error("Someone is renting this property already")]
    AlreadyReserved {},

    #[error("This rental is Not approved")]
    NotApproved {},

    #[error("This rental is started already")]
    RentalAlreadyStarted {},

    #[error("Rental is not activated yet")]
    RentalNotActivated {},

    #[error("Someone reserved this period already")]
    UnavailablePeriod {},

    #[error("Invalid input")]
    InvalidInput {},

    #[error("Rental period is too short")]
    LessThanMinimum {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },
}
