use dioxus::prelude::*;
use serde::Deserialize;
use crate::route::Route;

use crate::services::eth::*;

#[component]
pub fn Eth() -> Element {
    let mut contracts_funcs = use_resource(move || async move {
        get_server_data().await
    });
    let contracts = contracts_funcs.read_unchecked().clone();
    match contracts {
        // two tab for functions and state changing functions
        Some(Ok(value)) => {
                let functions = value.functions.clone();
                let copy_functions = use_signal(|| functions.clone());
                rsx! {
                    div {
                     class: "space-y-4 justify-center",
                         h1 { "{value.address}" }
                         hr{}
                         hr{}
                         h2 {
                            // text align center
                            class: "text-center",
                            "read Functions" }
                         ul {
                                for (index, func) in functions.iter().enumerate() {
                                    li {
                                            button{
                                                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                                                onclick: move |_| async move{
                                                    let clone = copy_functions.read()[index].clone();
                                                    call_read_function(clone.name,clone.selector).await;
                                                },
                                                "{func.name}"
                                            }
                                            p{
                                                // border class
                                                class: "border-2 border-gray-100",
                                                br{}
                                            }
                                    }
                                }
                            }
                        hr{}
                        hr{}
                        ul{
                            h2{"write Functions"}
                            if let Some(state_changing_functions) = &value.state_changing_functions {
                                for (index, func) in state_changing_functions.iter().enumerate() {
                                    li {
                                        h3{
                                            "{func.name}"
                                        }
                                        p{"{func.selector}"}
                                    }
                                }
                            } else {
                                // Handle the case when value.state_changing_functions is None
                                p{"no state changing functions found"}
                            }

                        }
                    }
                }
            },
        Some(Err(err)) => rsx! { "Error: {err}" },
        None => rsx! { "Loading..." },
    }
}



