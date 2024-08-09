use dioxus::prelude::*;
use tracing::info;

use crate::starknet_wrapper::provider::Network;

pub mod login;
pub mod eth;


#[server]
pub async fn set_global_net_work(network: Network) -> Result<bool, ServerFnError>{
    use crate::server_config::session;
    info!("set_global_net_work network: {:?}", network);
    let session: session::Session = extract().await.unwrap();
    let axum_session = session.axum_session;
    let session_id = axum_session.get_session_id();
    let session_id_str = session_id.to_string();
    let net_work_str = session_id_str + "network";
    info!("net_work_str: {:?}", net_work_str);
    axum_session.set(&net_work_str, network);
    Ok(true)
}