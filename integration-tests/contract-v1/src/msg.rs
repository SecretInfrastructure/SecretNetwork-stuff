use cosmwasm_std::{Binary, Coin, IbcTimeout, VoteOption};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Msg {
    Nop {},
    BankMsgSend {
        to_address: String,
        amount: Vec<Coin>,
    },
    StargateMsg {
        type_url: String,
        value: Binary,
    },
    StakingMsgDelegate {
        validator: String,
        amount: Coin,
    },
    StakingMsgUndelegate {
        validator: String,
        amount: Coin,
    },
    StakingMsgRedelegate {
        src_validator: String,
        dst_validator: String,
        amount: Coin,
    },
    GovVote {
        proposal_id: u64,
        vote: VoteOption,
    },
    DistributionMsgSetWithdrawAddress {
        address: String,
    },
    DistributionMsgWithdrawDelegatorReward {
        validator: String,
    },

    IbcMsgTransfer {
        channel_id: String,
        to_address: String,
        amount: Coin,
        timeout: IbcTimeout,
    },
    IbcMsgSendPacket {
        channel_id: String,
        data: Binary,
        timeout: IbcTimeout,
    },
    IbcMsgCloseChannel {
        channel_id: String,
    },
    WasmMsgInstantiate {
        code_id: u64,
        code_hash: String,
        msg: Binary,
        funds: Vec<Coin>,
        label: String,
    },
    WasmMsgExecute {
        contract_addr: String,
        code_hash: String,
        msg: Binary,
        funds: Vec<Coin>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    BankBalance {
        address: String,
        denom: String,
    },
    BankAllBalances {
        address: String,
    },
    StakingBondedDenom {},
    StakingAllDelegations {
        delegator: String,
    },
    StakingDelegation {
        delegator: String,
        validator: String,
    },
    StakingAllValidators {},
    StakingValidator {
        address: String,
    },
    Stargate {
        path: String,
        data: Binary,
    },
    IbcPortId {},
    IbcListChannels {
        port_id: Option<String>,
    },
    IbcChannel {
        channel_id: String,
        port_id: Option<String>,
    },
    WasmSmart {
        contract_addr: String,
        code_hash: String,
        msg: Binary,
    },
    WasmContractInfo {
        contract_addr: String,
    },
}
