// use serde_json::Value;
use serde::{Deserialize};

use log::error;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Response {
    jsonrpc: String,
    #[serde(alias = "result", alias = "error")]
    pub result: RequestObj,
    id: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    author: String,
    difficulty: String,
    extra_data: String,
    gas_limit: String,
    gas_used: String,
    hash: String,
    logs_bloom: String,
    miner: String,
    number: String,
    parent_hash: String,
    receipts_root: String,
    seal_fields: Option<Vec<String>>,
    sha3_uncles: String,
    signature: Option<String>,
    size: String,
    step: Option<StringOrI64>,
    state_root: String,
    timestamp: String,
    total_difficulty: String,
    base_fee_per_gas: Option<String>,
    pub transactions: Vec<String>,
    transactions_root: String,
    uncles: Vec<String>,
    mix_hash: Option<String>,
    nonce: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum StringOrI64 {
    String(String),
    I64(i64)
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    transaction_hash: String,
    transaction_index: String,
    block_hash: String,
    block_number: String,
    cumulative_gas_used: String,
    gas_used: String,
    effective_gas_price: Option<String>,
    from: Option<String>,
    to: Option<String>,
    contract_address: Option<String>,
    logs: Vec<Logs>,
    logs_bloom: String,
    status: String,
    #[serde(rename = "type")] 
    type_name: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: i64,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Logs {
    removed: bool,
    log_index: String,
    transaction_index: String,
    transaction_hash: String,
    block_hash: String,
    block_number: String,
    address: String,
    data: String,
    topics: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RequestObj {
    MaxBlock(String),
    Block(Box<Block>),
    Receipt(Box<Receipt>),
    Error(Error),
}

pub fn from_hex_to_int(num:&str) -> u64 {
    let without_prefix = num.trim_start_matches("0x");

    u64::from_str_radix(without_prefix, 16).unwrap()
}

pub fn get_hashes(json: Response) -> Vec<String> {
    if let RequestObj::Block(block) = json.result {
        block.transactions
    } else if let RequestObj::Error(err) = json.result {
        error!("error from EVM with code {}: {}", err.code, err.message);
        vec![]
    } else {
        vec![]
    }
}

pub fn get_gas(json: Response) -> String {
    if let RequestObj::Receipt(transaction) = json.result {
        transaction.gas_used
    } else if let RequestObj::Error(err) = json.result {
        error!("error from EVM with code {}: {}", err.code, err.message);
        "".into()
    } else {
        "".into()
    }
}
