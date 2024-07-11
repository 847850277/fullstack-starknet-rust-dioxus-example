use dioxus::prelude::*;

#[component]
pub fn Contracts() -> Element {
    require_login!();

    rsx! {
        div {
            class: "space-y-4 w-2/5",
            p {
                class: "text",
                "11111"
            }
        }
    }
}