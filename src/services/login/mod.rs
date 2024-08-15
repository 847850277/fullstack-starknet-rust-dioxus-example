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
use tracing::info;
use crate::services::eth::Contract;

#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub struct User {
    pub security: String,
    pub address: String,
}


//new user
impl User {
    pub fn new(security: String, address: String) -> User {
        User {
            security,
            address,
        }
    }
}


#[server]
pub async fn get_login_data() -> Result<bool, ServerFnError> {
    use crate::server_config::session;

    let session: session::Session = extract().await.unwrap();
    log::debug!("session: {:?}", session);
    let axum_session = session.axum_session;

    let session_id = axum_session.get_session_id();
    let session_id_str = session_id.to_string();
    let login = axum_session.get::<(bool, User)>(&session_id_str);
    info!("login: {:?}", login);
    match login {
        Some((login, user)) => {
            return Ok(login);
        }
        None => {
            return Ok(false);
        }
    }
}


#[server]
pub async fn login_page(address: String, private_key: String) -> Result<bool, ServerFnError> {
    use crate::server_config::session;
    use sqlx::Row;
    use crate::starknet_wrapper::provider::Network;

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
    ));
    let chain_id = provider.chain_id().await.unwrap();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(&private_key).unwrap(),
    ));
    let address_field_element = FieldElement::from_hex_be(&address).unwrap();
    //let address = felt!(&address);

    let account = SingleOwnerAccount::new(provider, signer, address_field_element, chain_id, ExecutionEncoding::New);
    let account_1 = Arc::new(account);
    let result = account_1.get_nonce().await;

    let session: session::Session = extract().await.unwrap();
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

            let axum_session = session.axum_session;
            log::debug!("axum_session: {:?}", axum_session);
            let session_id = axum_session.get_session_id();
            let session_id_str = session_id.to_string();
            // session set tuple bool user
            let login = (true, User::new(private_key, address));
            axum_session.set(&session_id_str, login);

            // if not network_key then set global network testnet
            let network_key = session_id_str + "network";
            let network: Option<Network> = axum_session.get(&network_key);
            match network {
                Some(_) => {
                    return Ok(true);
                }
                None => {
                    let network = Network::Testnet;
                    axum_session.set(&network_key, network);
                }
            }

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


