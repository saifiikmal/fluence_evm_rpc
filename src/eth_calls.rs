use crate::curl_request_res;
use crate::eth_utils::{check_response_string, check_response_transaction_string, get_nonce};
use crate::fce_results::{JsonRpcResult, JsonRpcTransactionResult};
use crate::jsonrpc_helpers::Request;
use crate::types::TxCall;

use ethereum_types::H256;
use jsonrpc_core as rpc;
use marine_rs_sdk::marine;
use serde_json::json;

pub fn serialize<T: serde::Serialize>(t: &T) -> rpc::Value {
    serde_json::to_value(t).expect("Types never fail to serialize.")
}

pub fn eth_call(url: String, tx: TxCall, tag: String) -> JsonRpcResult {
    let method = "eth_call".to_string();

    let tx_call_serial = serialize(&tx);
    let tag_serial = serialize(&tag);
    let params: rpc::Value = json!(vec![tx_call_serial, tag_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id)
}

// pub fn eth_send_transaction(url: String, tx: TxCall) -> JsonRpcResult {
//     let method = "eth_sendTransaction".to_string();

//     let tx_call_serial = serialize(&tx);
//     let params: rpc::Value = json!(vec![tx_call_serial]);

//     let id = get_nonce();

//     let curl_args = Request::new(method, params, id).as_sys_string(&url);
//     let response = curl_request_res(curl_args).unwrap();

//     check_response_string(response, &id)
// }

pub fn eth_get_transaction_receipt(url: String, trans_hash: H256) -> JsonRpcResult {
    let method = "eth_getTransactionReceipt".to_string();

    let trans_hash_serial = serialize(&trans_hash);
    let params: rpc::Value = json!(vec![trans_hash_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id)
}

#[marine]
pub fn eth_get_latest_block_number(url: String) -> JsonRpcResult {
    let method = "eth_blockNumber".to_string();
    let params: rpc::Value = json!([]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    log::info!("{}", response);
    check_response_string(response, &id)
}

#[marine]
pub fn eth_get_block_by_number(url: String, block_in_hex: String) -> JsonRpcTransactionResult {
    let method = "eth_getBlockByNumber".to_string();

    let block_serial = serialize(&block_in_hex);
    let is_hydrated_serial = serialize(&true);
    let params: rpc::Value = json!(vec![block_serial, is_hydrated_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_transaction_string(response, &id)
}

#[marine]
pub fn eth_send_raw_transaction(url: String, signed_tx: String) -> JsonRpcResult {
    let method = "eth_sendRawTransaction".to_string();

    let signed_tx_serial = serialize(&signed_tx);
    let params: rpc::Value = json!(vec![signed_tx_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id)
}

#[marine]
pub fn eth_get_balance(url: String, add: String) -> JsonRpcResult {
    let method = "eth_getBalance".to_string();

    let add_serial = serialize(&add);
    let tag_serial = serialize(&"latest".to_string());
    let params: rpc::Value = json!(vec![add_serial, tag_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    log::info!("{}", response);
    check_response_string(response, &id)
}
