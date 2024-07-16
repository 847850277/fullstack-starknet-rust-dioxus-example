use dioxus::prelude::*;
use serde::{Deserialize, Serialize};


pub type GatewayResult<T> = Result<T, GatewayError>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GatewayError {
    FailedDeserialization,
    FailedAta,
    FailedRegister,
    TransactionTimeout,
    NetworkUnavailable,
    AccountNotFound,
    // SimulationFailed,
    RequestFailed,
    ProgramBuilderFailed,
    WalletAdapterDisconnected,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiTokenAmount {
    pub decimals: u8,
    pub name: String,
}

pub fn use_ore_supply() -> Resource<GatewayResult<UiTokenAmount>> {
    use_resource(move || {
        async move {
            println!("use_ore_supply");
            get_token_supply()
                .await
                .map_err(|_| GatewayError::Unknown)
        }
    })
}

async fn get_token_supply() -> Result<UiTokenAmount, GatewayError> {
    // Replace this with the actual implementation
    Ok(UiTokenAmount {
        decimals: 18,
        name: String::from("Test Token"),
    })
}