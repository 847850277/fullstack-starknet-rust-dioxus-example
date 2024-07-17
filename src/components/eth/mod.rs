use dioxus::prelude::*;
use serde::Deserialize;

use crate::services::eth::*;

#[component]
pub fn Eth() -> Element {
    let mut contracts_funcs = use_resource(move || async move {
        get_server_data().await
    });
    match &*contracts_funcs.read_unchecked() {
        // two tab for functions and state changing functions
        Some(Ok(value)) => rsx! {
                div {
                 class: "space-y-4 justify-center",
                 h1 { "{value.address}" }
                 h2 { "Functions" }
                 ul {
                        // for value.functions.iter().map(|func| {
                        //     return rsx! {
                        //         li {
                        //             "{func.name}"
                        //         }
                        //     }
                        // })
                        for (index, func) in value.functions.iter().enumerate() {
                            li {
                                    "{func.name}"
                            }
                        }
                    }
                }
            },
            //rsx! { "{value:?}" },
        Some(Err(err)) => rsx! { "Error: {err}" },
        None => rsx! { "Loading..." },
    }
}
