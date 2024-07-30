use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::views::home::Home;
use super::views::tickets::Tickets;
use super::views::login::Login;
use super::views::eth::Eth;
use super::views::navbar::NavBar;
use super::views::page_not_found::PageNotFound;



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