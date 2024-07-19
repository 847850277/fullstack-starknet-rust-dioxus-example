use std::i64;
use std::sync::Arc;
use dioxus::prelude::*;

use reqwest::Url;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::accounts::{Account, ExecutionEncoding, SingleOwnerAccount,ConnectedAccount};
use starknet::core::chain_id;
use starknet::core::types::{BlockId, BlockTag};
use starknet::core::types::FieldElement;
use starknet::signers::{LocalWallet, SigningKey};


use crate::login;

#[server]
pub async fn login_page(address: String, private_key: String) -> Result<bool, ServerFnError> {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));
    let chain_id = provider.chain_id().await.unwrap();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(&private_key).unwrap(),
    ));
    let address = FieldElement::from_hex_be(&address).unwrap();
    //let address = felt!(&address);

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);
    let account_1 = Arc::new(account);
    let result = account_1.get_nonce().await;
    match result {

        Ok(nonce) => {
            //把账号和密码缓存
            // let auth: crate::auth::Session = extract().await?;
            // let decimal = nonce.to_big_decimal(0);
            // let string = decimal.to_string();
            // let id = string.parse().unwrap();
            // auth.login_user(id);
            Ok(true)
        }
        Err(e) => {
            return Err(ServerFnError::new("账号或者密码不正确"));
        },
    }
}


