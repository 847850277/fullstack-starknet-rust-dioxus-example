use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use reqwest::Error;
use serde_json::json;
use tracing::info;
use starknet::core::types::FieldElement;
use hex;
use crate::my_starknet::provider::{create_jsonrpc_client, Network};
use crate::my_starknet::contract::call_contract_read_function;



#[server]
pub async fn get_server_data() -> Result<Contract, ServerFnError> {
    let url = "https://sepolia.voyager.online/api/contract/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7/functions";
    let response = reqwest::get(url).await.map_err(|err| ServerFnError::new(err.to_string()))?;
    let body = response.text().await.map_err(|err| ServerFnError::new(err.to_string()))?;
    let contract: Contract = serde_json::from_str(&body).map_err(|err| ServerFnError::new(err.to_string()))?;
    println!("{:?}", contract);
    Ok(contract)
}


#[server]
pub async fn call_read_function(my_selector: String, contract_address: String) -> Result<Contract, ServerFnError>{
    info!("call_read_function selector: {}, contract_address: {}", my_selector, contract_address);
    // call the read contract function
    let contract_address = FieldElement::from_hex_be(&contract_address)
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    // change my_select string to hex in rust
    let hex_string = hex::encode(my_selector);
    let selector = FieldElement::from_hex_be(&hex_string)
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    let response = call_contract_read_function(create_jsonrpc_client(Network::Testnet), contract_address, selector,vec![]).await;
    dbg!(response);

    return Ok(Contract{
        address: "".to_string(),
        functions: vec![],
        state_changing_functions: None,
        abi: vec![],
        proxy_metadata: None,
    })
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Contract {
    pub address: String,
    pub functions: Vec<Function>,
    #[serde(rename = "stateChangingFunctions")]
    pub state_changing_functions: Option<Vec<StateChangingFunction>>,
    pub abi: Vec<Abi>,
    pub proxy_metadata: Option<ProxyMetadata>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Function {
    pub selector: String,
    pub name: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct StateChangingFunction {
    pub selector: String,
    pub name: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Abi {
    pub name: String,
    #[serde(rename = "type")]
    pub abi_type: String,
    pub interface_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ProxyMetadata {
    pub proxy_address: String,
    pub functions: Vec<String>,
    pub state_changing_functions: Vec<String>,
    pub abi: Vec<String>,
    #[serde(rename = "type")]
    pub metadata_type: String,
}
