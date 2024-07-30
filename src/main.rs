#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod views;
mod route;
mod services;
mod starknet_wrapper;

use views::{Home, Login, NavBar, Tickets};

use route::Route;

#[cfg(feature = "server")]
mod server_config;

fn main() {
    //wasm_logger::init(wasm_logger::Config::default());
    //dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

    #[cfg(feature = "web")]
    tracing_wasm::set_as_global_default();

    // #[cfg(feature = "server_config")]
    // tracing_subscriber::fmt::init();

    #[cfg(feature = "server")]
    server_config::start::start(App);

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

