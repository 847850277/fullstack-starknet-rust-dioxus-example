use std::i64;
use std::ops::Deref;
use std::sync::Arc;

use dioxus::prelude::*;
use reqwest::Url;
use starknet::accounts::{Account, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::chain_id;
use starknet::core::types::{BlockId, BlockTag};
use starknet::core::types::FieldElement;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::signers::{LocalWallet, SigningKey};

#[server]
pub async fn login_page(address: String, private_key: String) -> Result<bool, ServerFnError> {
    use crate::server_config::session;
    use sqlx::Row;

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

    //TODO get set
    let session: session::Session = extract().await.unwrap();
    let session_id = session.axum_session.get("session_id").unwrap_or(0);
    log::debug!("session_id: {:?}", session_id);

    match result {

        Ok(nonce) => {


            let dbp = session.dbp;
            // insert into table users
            let insert_sql = format!(r#"
                INSERT INTO users (security, address) VALUES ('{}', '{}')
            "#, private_key, address);
            let row = sqlx::query(&insert_sql)
                .execute(dbp.deref())
                .await
                .map_err(|err| err.to_string());
            match row {
                Ok(row) => {
                    return Ok(true);
                }
                Err(e) => {
                    return Err(ServerFnError::new("账号或者密码不正确"));
                },
            }

            Ok(true)
        }
        Err(e) => {
            return Err(ServerFnError::new("账号或者密码不正确"));
        },
    }
}


