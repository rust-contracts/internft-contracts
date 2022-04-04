use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid CW721 Receive Message")]
    InvalidCw721ReceiveMsg {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("No Staked Tokens")]
    NoStakedToken {},

    #[error("Token Already Staked")]
    TokenAlreadyStaked {},

    #[error("Invalid Staking Type")]
    InvalidStakingType {},
}
