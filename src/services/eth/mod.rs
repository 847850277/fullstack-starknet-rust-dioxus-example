use std::collections::HashMap;
use std::sync::Arc;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use reqwest::Error;
use serde_json::json;
use tracing::info;
use starknet::core::types::FieldElement;
use crate::starknet_wrapper::provider::{create_jsonrpc_client, Network};
use crate::starknet_wrapper::contract::call_contract_read_function;

pub static MAINNET_CONTRACT: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";

//pub static MAINNET_CONTRACT: &str = "0x077c648eeda3db3935c1e6ed69b51a2c28d7addbead74b3166589a0d166aee5e";
pub static TESTNET_CONTRACT: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
pub static MAINNET_BASE_URL: &str = "https://voyager.online/";

pub static TESTNET_BASE_URL: &str = "https://sepolia.voyager.online/";

#[server]
pub async fn get_server_data() -> Result<Contract, ServerFnError> {
    use crate::server_config::session;

    let session: session::Session = extract().await.unwrap();
    info!("session: {:?}", session);
    let network_key = session.axum_session.get_session_id().to_string() + "network";
    let network: Network = session.axum_session.get(&network_key).unwrap();
    info!("network: {:?}", network);
    match network {
        Network::Mainnet => {
            let url = MAINNET_BASE_URL.to_string() + "api/contract/" + MAINNET_CONTRACT + "/functions";
            let response = reqwest::get(url).await.map_err(|err| ServerFnError::new(err.to_string()))?;
            let body = response.text().await.map_err(|err| ServerFnError::new(err.to_string()))?;
            let contract: Contract = serde_json::from_str(&body).map_err(|err| ServerFnError::new(err.to_string()))?;
            return Ok(contract);
        }
        Network::Testnet => {
            let url = TESTNET_BASE_URL.to_string() + "api/contract/" + TESTNET_CONTRACT + "/functions";
            let response = reqwest::get(url).await.map_err(|err| ServerFnError::new(err.to_string()))?;
            let body = response.text().await.map_err(|err| ServerFnError::new(err.to_string()))?;
            let contract: Contract = serde_json::from_str(&body).map_err(|err| ServerFnError::new(err.to_string()))?;
            return Ok(contract);
        }
    }
}


#[server]
pub async fn call_read_function(my_selector: String, contract_address: String) -> Result<String, ServerFnError>{
    use crate::server_config::session;
    use crate::services::login::User;

    info!("call_read_function selector: {}, contract_address: {}", my_selector, contract_address);
    let session: session::Session = extract().await.unwrap();
    log::debug!("session: {:?}", session);
    let network_key = session.axum_session.get_session_id().to_string() + "network";
    info!("network_key: {:?}", network_key);
    let network: Network = session.axum_session.get(&network_key).unwrap();
    info!("network: {:?}", network);
    let session_id_str = session.axum_session.get_session_id().to_string();
    info!("session_id: {:?}", session_id_str);
    let axum_session = session.axum_session;
    info!("axum_session: {:?}", axum_session);
    let login:Option<User> = axum_session.get(&session_id_str);
    info!("login: {:?}", login);
    let session_store = axum_session.get_store();
    info!("session_store: {:?}", session_store);


    let contract_address = FieldElement::from_hex_be(&contract_address)
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let response = call_contract_read_function(create_jsonrpc_client(network), contract_address, my_selector,vec![]).await;
    let response = serde_json::to_string(&response).map_err(|err| ServerFnError::new(err.to_string()))?;
    return Ok(response);
}


#[server]
pub async fn call_write_function(my_selector: String, contract_address: String,param : HashMap<String,String>) -> Result<String, ServerFnError>{
    use crate::server_config::session;
    use crate::services::login::User;
    use starknet::providers::Provider;
    use starknet::accounts::{Account, Call, ExecutionEncoding, SingleOwnerAccount};
    use starknet::signers::{LocalWallet, SigningKey};
    use starknet::core::utils::get_selector_from_name;

    info!("call_write_function selector: {}, contract_address: {}", my_selector, contract_address);
    info!("param: {:?}", param);
    let response = json!({
        "status": "success",
        "message": "write function called"
    });

    let session: session::Session = extract().await.unwrap();
    let session_id = session.axum_session.get_session_id().to_string();
    info!("session: {:?}", session);
    info!("session_id: {:?}", session_id);
    let network_key = session_id.clone() + "network";
    let network: Network = session.axum_session.get(&network_key).unwrap();
    let contract_address = FieldElement::from_hex_be(&contract_address)
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let provider = create_jsonrpc_client(network);
    let login:Option<User> = session.axum_session.get(&session_id);
    let private_key = login.clone().unwrap().security;
    let address = login.clone().unwrap().address;
    let address = FieldElement::from_hex_be(&address).unwrap();
    let chain_id = provider.chain_id().await.unwrap();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(&private_key).unwrap(),
    ));
    let account = SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);
    let account_1 = Arc::new(account);
    let input_address = param.get("recipient").unwrap();
    let account_2_address = FieldElement::from_hex_be(input_address).unwrap();
    let high = FieldElement::from_hex_be("10000").unwrap();
    let low = FieldElement::from_hex_be("0").unwrap();
    let transfer_response = account_1.execute(vec![Call {
        to: contract_address,
        selector: get_selector_from_name(&my_selector).unwrap(),
        calldata: vec![account_2_address, high, low],
    }]).send().await.unwrap();
    // transfer_response to string
    //let response = transfer_response.to_string();
    let response = serde_json::to_string(&transfer_response).map_err(|err| ServerFnError::new(err.to_string()))?;
    return Ok(response);
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
