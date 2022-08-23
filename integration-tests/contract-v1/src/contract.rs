use cosmwasm_std::{
    entry_point, to_binary, to_vec, AllBalanceResponse, AllDelegationsResponse,
    AllValidatorsResponse, BalanceResponse, BankMsg, BankQuery, Binary, BondedDenomResponse,
    ChannelResponse, ContractInfoResponse, CosmosMsg, DelegationResponse, Deps, DepsMut,
    DistributionMsg, Empty, Env, GovMsg, IbcMsg, IbcQuery, ListChannelsResponse, MessageInfo,
    PortIdResponse, QueryRequest, Response, StakingMsg, StakingQuery, StdResult, ValidatorResponse,
    WasmMsg, WasmQuery,
};

use crate::msg::{Msg, QueryMsg};

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Msg) -> StdResult<Response> {
    return handle_msg(deps, env, info, msg);
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Msg) -> StdResult<Response> {
    return handle_msg(deps, env, info, msg);
}

fn handle_msg(_deps: DepsMut, _env: Env, _info: MessageInfo, msg: Msg) -> StdResult<Response> {
    match msg {
        Msg::Nop {} => {
            return Ok(Response::new());
        }
        Msg::BankMsgSend { to_address, amount } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Bank(BankMsg::Send { to_address, amount }))
            );
        }
        Msg::StargateMsg { type_url, value } => {
            return Ok(Response::new().add_message(CosmosMsg::Stargate { type_url, value }));
        }
        Msg::StakingMsgDelegate { validator, amount } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Staking(StakingMsg::Delegate {
                    validator,
                    amount,
                })),
            );
        }
        Msg::StakingMsgUndelegate { validator, amount } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Staking(StakingMsg::Undelegate {
                    validator,
                    amount,
                })),
            );
        }
        Msg::StakingMsgRedelegate {
            src_validator,
            dst_validator,
            amount,
        } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Staking(StakingMsg::Redelegate {
                    src_validator,
                    dst_validator,
                    amount,
                })),
            );
        }
        Msg::GovVote { proposal_id, vote } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Gov(GovMsg::Vote { proposal_id, vote }))
            );
        }
        Msg::DistributionMsgSetWithdrawAddress { address } => {
            return Ok(Response::new().add_message(CosmosMsg::Distribution(
                DistributionMsg::SetWithdrawAddress { address },
            )));
        }
        Msg::DistributionMsgWithdrawDelegatorReward { validator } => {
            return Ok(Response::new().add_message(CosmosMsg::Distribution(
                DistributionMsg::WithdrawDelegatorReward { validator },
            )));
        }
        Msg::IbcMsgTransfer {
            channel_id,
            to_address,
            amount,
            timeout,
        } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Ibc(IbcMsg::Transfer {
                    channel_id,
                    to_address,
                    amount,
                    timeout,
                })),
            );
        }
        Msg::IbcMsgSendPacket {
            channel_id,
            data,
            timeout,
        } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Ibc(IbcMsg::SendPacket {
                    channel_id,
                    data,
                    timeout,
                })),
            );
        }
        Msg::IbcMsgCloseChannel { channel_id } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Ibc(IbcMsg::CloseChannel { channel_id }))
            );
        }
        Msg::WasmMsgInstantiate {
            code_id,
            code_hash,
            msg,
            funds,
            label,
        } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Instantiate {
                    code_id,
                    code_hash,
                    msg,
                    funds,
                    label,
                })),
            );
        }
        Msg::WasmMsgExecute {
            contract_addr,
            code_hash,
            msg,
            funds,
        } => {
            return Ok(
                Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr,
                    code_hash,
                    msg,
                    funds,
                })),
            );
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Stargate { path, data } => {
            return Ok(to_binary(
                &deps
                    .querier
                    .raw_query(&to_vec(&QueryRequest::<Empty>::Stargate { path, data })?)
                    .unwrap()
                    .unwrap(),
            )?);
        }
        QueryMsg::BankBalance { address, denom } => {
            return Ok(to_binary(&deps.querier.query::<BalanceResponse>(
                &QueryRequest::Bank(BankQuery::Balance { address, denom }),
            )?)?);
        }
        QueryMsg::BankAllBalances { address } => {
            return Ok(to_binary(&deps.querier.query::<AllBalanceResponse>(
                &QueryRequest::Bank(BankQuery::AllBalances { address }),
            )?)?);
        }
        QueryMsg::StakingBondedDenom {} => {
            return Ok(to_binary(&deps.querier.query::<BondedDenomResponse>(
                &QueryRequest::Staking(StakingQuery::BondedDenom {}),
            )?)?);
        }
        QueryMsg::StakingAllDelegations { delegator } => {
            return Ok(to_binary(&deps.querier.query::<AllDelegationsResponse>(
                &QueryRequest::Staking(StakingQuery::AllDelegations { delegator }),
            )?)?);
        }
        QueryMsg::StakingDelegation {
            delegator,
            validator,
        } => {
            return Ok(to_binary(&deps.querier.query::<DelegationResponse>(
                &QueryRequest::Staking(StakingQuery::Delegation {
                    delegator,
                    validator,
                }),
            )?)?);
        }
        QueryMsg::StakingAllValidators {} => {
            return Ok(to_binary(&deps.querier.query::<AllValidatorsResponse>(
                &QueryRequest::Staking(StakingQuery::AllValidators {}),
            )?)?);
        }
        QueryMsg::StakingValidator { address } => {
            return Ok(to_binary(&deps.querier.query::<ValidatorResponse>(
                &QueryRequest::Staking(StakingQuery::Validator { address }),
            )?)?);
        }
        QueryMsg::IbcPortId {} => {
            return Ok(to_binary(&deps.querier.query::<PortIdResponse>(
                &QueryRequest::Ibc(IbcQuery::PortId {}),
            )?)?);
        }
        QueryMsg::IbcListChannels { port_id } => {
            return Ok(to_binary(&deps.querier.query::<ListChannelsResponse>(
                &QueryRequest::Ibc(IbcQuery::ListChannels { port_id }),
            )?)?);
        }
        QueryMsg::IbcChannel {
            channel_id,
            port_id,
        } => {
            return Ok(to_binary(&deps.querier.query::<ChannelResponse>(
                &QueryRequest::Ibc(IbcQuery::Channel {
                    channel_id,
                    port_id,
                }),
            )?)?);
        }
        QueryMsg::WasmSmart {
            contract_addr,
            code_hash,
            msg,
        } => {
            return Ok(to_binary(&deps.querier.query::<Binary /* TODO fix */>(
                &QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr,
                    code_hash,
                    msg,
                }),
            )?)?);
        }
        QueryMsg::WasmContractInfo { contract_addr } => {
            return Ok(to_binary(&deps.querier.query::<ContractInfoResponse>(
                &QueryRequest::Wasm(WasmQuery::ContractInfo { contract_addr }),
            )?)?);
        }
    }
}
