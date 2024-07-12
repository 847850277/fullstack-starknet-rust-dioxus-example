use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::components::home::Home;
use super::components::tickets::Tickets;
use super::components::login::Login;
use super::components::eth::Eth;
use super::components::navbar::NavBar;
use super::components::page_not_found::PageNotFound;



#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/tickets")]
    Tickets {},
    #[route("/login")]
    Login {},
    #[route("/eth")]
    Eth {},
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}