use crate::Route;
use dioxus::prelude::*;
use tracing::info;
use crate::services::set_global_net_work;
use crate::starknet_wrapper::provider::Network;

#[component]
pub fn NavBar() -> Element {
    //let mut net_work = use_signal(|| Network::Testnet);
    use_resource(move || async move {
        set_global_net_work(Network::Testnet).await
    });

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
                                let net_work: Network = evt.value().parse().unwrap();
                                 use_resource(move || async move {
                                    set_global_net_work(net_work).await
                                });
                            },
                            // onchange: change_net_work,
                            option {
                                value: "Testnet",
                                "Testnet"
                            }
                            option {
                                value: "Mainnet",
                                "Mainnet"
                            }
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}