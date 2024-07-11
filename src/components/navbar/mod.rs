use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
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
                        Link { to: Route::Contracts {}, "contracts" }
                    }
                    div {
                        class: "px-4 py-2 hover:bg-blue-700 justify-center",
                        Link { to: Route::Home {}, "Create tickets" }
                    }
                    div {
                        class: "px-4 py-2 hover:bg-blue-700 justify-center",
                        Link { to: Route::Tickets {}, "View tickets" }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}