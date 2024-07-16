#![allow(non_snake_case, unused)]

use std::sync::{Arc, Mutex};

use dioxus::prelude::{
    server_fn::middleware::{self, Layer},
    *,
};
use tokio::sync::OnceCell;
use tracing::{info, Level};
mod error;


#[cfg(feature = "server")]
use pwhash::bcrypt;
use tracing::metadata::LevelFilter;

mod components;
mod route;
mod services;

use components::{Home, Login, NavBar, Tickets};

use route::Route;


fn main() {
    //wasm_logger::init(wasm_logger::Config::default());
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new(false));
    rsx! {
        div {
            class: "dark flex justify-center",
            Router::<Route> {}
        }
    }
}



#[server]
pub async fn login(username: String, password: String) -> Result<i64, ServerFnError> {

    Ok(1)
}
