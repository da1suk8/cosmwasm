use cosmwasm_std::{
    callable_point, entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    Storage,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, NumberResponse, QueryMsg};

const KEY: &[u8] = b"number";

mod __tete {

    #[derive(serde::Serialize, serde::Deserialize)]
    struct CalleeDict {
        #[serde(flatten)]
        inner: std::collections::HashMap<String, bool>,
    }

    #[no_mangle]
    extern "C" fn _is_callee_function_read_only(arg: u32) -> u32 {
        let vec_arg_idents: Vec<u8> = unsafe {
            cosmwasm_std::memory::consume_region(arg as *mut cosmwasm_std::memory::Region)
        };
        let _arg_idents: String = cosmwasm_std::from_slice(&vec_arg_idents).unwrap();

        let mut tete = CalleeDict {
            inner: std::collections::HashMap::new(),
        };

        tete.inner.insert(String::from("add"), false);
        tete.inner.insert(String::from("mul"), false);
        tete.inner.insert(String::from("sub"), false);
        tete.inner.insert(String::from("number"), true);

        // let result = func_context.get(&arg_idents).unwrap();
        let vec_result = cosmwasm_std::to_vec(&tete).unwrap();
        cosmwasm_std::memory::release_buffer(vec_result) as u32
    }
}

fn write(storage: &mut dyn Storage, value: i32) {
    storage.set(KEY, &value.to_be_bytes())
}

fn read(storage: &dyn Storage) -> Result<i32, ContractError> {
    let vec_value = storage.get(KEY).ok_or(ContractError::StorageError)?;
    let array_value: [u8; 4] = [vec_value[0], vec_value[1], vec_value[2], vec_value[3]];
    Ok(i32::from_be_bytes(array_value))
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    write(deps.storage, msg.value);
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Add { value } => handle_add(deps, value),
        ExecuteMsg::Sub { value } => handle_sub(deps, value),
        ExecuteMsg::Mul { value } => handle_mul(deps, value),
    }
}

fn handle_add(deps: DepsMut, by: i32) -> Result<Response, ContractError> {
    let value = read(deps.storage)?;
    let new_value = value.checked_add(by).ok_or(ContractError::Overflow)?;
    write(deps.storage, new_value);
    Ok(Response::default())
}

fn handle_sub(deps: DepsMut, by: i32) -> Result<Response, ContractError> {
    let value = read(deps.storage)?;
    let new_value = value.checked_sub(by).ok_or(ContractError::Overflow)?;
    write(deps.storage, new_value);
    Ok(Response::default())
}

fn handle_mul(deps: DepsMut, by: i32) -> Result<Response, ContractError> {
    let value = read(deps.storage)?;
    let new_value = value.checked_mul(by).ok_or(ContractError::Overflow)?;
    write(deps.storage, new_value);
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Number {} => Ok(to_binary(&query_number(deps)?)?),
    }
}

fn query_number(deps: Deps) -> Result<NumberResponse, ContractError> {
    let value = read(deps.storage)?;
    Ok(NumberResponse { value })
}

#[callable_point]
fn add(deps: DepsMut, _env: Env, by: i32) {
    handle_add(deps, by).unwrap();
}

#[callable_point]
fn sub(deps: DepsMut, _env: Env, by: i32) {
    handle_sub(deps, by).unwrap();
}

#[callable_point]
fn mul(deps: DepsMut, _env: Env, by: i32) {
    handle_mul(deps, by).unwrap();
}

#[callable_point]
fn number(deps: Deps, _env: Env) -> i32 {
    read(deps.storage).unwrap()
}
