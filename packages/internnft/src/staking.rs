use cosmwasm_std::{Addr, Binary};
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub nft_contract_addr: Addr,
    pub terrand_addr: Addr,
    pub owner: Addr,
    pub stamina_constant: u64,
    pub exp_constant: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StakingInfo {
    pub staked: bool,
    pub last_action_block_time: u64,
    pub current_stamina: u64,
    pub token_id: String,
    pub owner: Addr,
    pub staking_type: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub nft_contract_addr: Addr,
    pub terrand_addr: Addr,
    pub stamina_constant: u64,
    pub exp_constant: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Allows this contract to be on the receiving end of a SendNft{contract, token_id, msg} call
    /// to the nft contract. The same thing as sending CW721 tokens to a contract.
    Receive(Cw721ReceiveMsg),
    UpdateConfig {
        nft_contract_addr: Addr,
        terrand_addr: Addr,
        owner: Addr,
        stamina_constant: u64,
        exp_constant: u64,
    },
    /// Allows the calling user to withdraw the specified nft if they own it.
    WithdrawNft { token_id: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Cw721HookMsg {
    Stake { staking_type: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetStakingInfo { token_id: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ContractQuery {
    //loterra terrand
    GetRandomness { round: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct GetRandomResponse {
    pub randomness: Binary,
    pub worker: String,
}
