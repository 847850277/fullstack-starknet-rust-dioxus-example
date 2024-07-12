use dioxus::prelude::*;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct Response {
    address: String,
    stateChangingFunctions: Vec<Function>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct Function{
    selector: String,
    name: String,
    vec: Vec<param>
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct param{
    name: String,
    #[serde(rename = "type")]
    param_type: String,
}

#[server]
pub async fn get_eth_write_fun(contract_address: String) -> Result<Function, ServerFnError> {

    let response = reqwest::get("https://sepolia.voyager.online/api/contract/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7/functions").await?;
    let data: Response = response.json().await?;
    // Placeholder return value
    return Ok(Function {
        selector: data.stateChangingFunctions[0].selector.clone(),
        name: data.stateChangingFunctions[0].name.clone(),
        //vec: data.stateChangingFunctions[0].parameters.clone(),
        vec: vec![],
    });

}