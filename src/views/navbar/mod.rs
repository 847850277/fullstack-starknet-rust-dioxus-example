use crate::Route;
use dioxus::prelude::*;
use tracing::info;
use crate::starknet_wrapper::provider::Network;

#[component]
pub fn NavBar() -> Element {
    let mut net_work = use_signal(|| Network::Testnet);

    rsx! {
        div {
            class: "space-y-4 w-full justify-center",
            nav { id: "navbar",
                class: "flex bg-blue-500 justify-center",
                div {
                    class: "flex justify-center",
                    div {
                        class: "px-4 py-2 hover:bg-blue-700 justify-center",
                        Link { to: Route::Login {}, "Login" }
                    }
                    div {
                        class: "px-4 py-2 hover:bg-blue-700 justify-center",
                        Link { to: Route::Eth {}, "eth" }
                    }
                    div {
                        class: "px-4 py-2 hover:bg-blue-700 justify-center",
                        Link { to: Route::Home {}, "Create tickets" }
                    }
                    div {
                        class: "px-4 py-2 hover:bg-blue-700 justify-center",
                        Link { to: Route::Tickets {}, "View tickets" }
                    }
                    div {
                        class: "px-4 py-2 hover:bg-blue-700 justify-center",
                        select {
                            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                            id: "network_config",
                            oninput: move |evt| {
                                // info!("Network changed to: {:?}", evt.value());
                                net_work.set(evt.value().parse().unwrap());
                            },
                            option {
                                value: "Mainnet",
                                "Mainnet"
                            }
                            option {
                                value: "Testnet",
                                "Testnet"
                            }
                        }
                    }
                    p{ "{net_work.read()}" }
                }
            }
            Outlet::<Route> {}
        }
    }
}