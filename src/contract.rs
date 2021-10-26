use crate::msg::ExecuteMsg;
use crate::msg::InstantiateMsg;
use cosmwasm_std::DepsMut;

use cosmwasm_std::{entry_point, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:terra-mooon-forever-locking-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::LockForever {} => handle_lock_forever(deps, info),
    }
}

pub fn handle_lock_forever(_deps: DepsMut, _info: MessageInfo) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "locked_forever"))
}
