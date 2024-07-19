#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod components;
mod route;
mod services;
mod starknet;

use components::{Home, Login, NavBar, Tickets};

use route::Route;


fn main() {
    //wasm_logger::init(wasm_logger::Config::default());
    //dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

    #[cfg(feature = "web")]
    tracing_wasm::set_as_global_default();

    #[cfg(feature = "server")]
    tracing_subscriber::fmt::init();

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
