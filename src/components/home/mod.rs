use dioxus::prelude::*;

use crate::create_ticket;

#[component]
pub fn Home() -> Element {
    require_login!();

    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    let mut title = use_signal(|| "".to_string());
    let mut body = use_signal(|| "".to_string());

    let submit = move |_| async move {
        create_ticket(title.to_string().clone(), body.to_string().clone()).await;
    };

    rsx! {
        div {
            class: "space-y-4 w-2/5",

            div {
                class: "space-y-4 justify-center",
                input {
                    class: "block w-full p-2 border border-gray-300",
                    placeholder: "Enter title...",
                    oninput: move |text| title.set(text.value())
                }
                input {
                    class: "block w-full p-2 border border-gray-300",
                    placeholder: "Enter body...",
                    oninput: move |text| body.set(text.value())
                }
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: submit,
                    "Submit ticket"
                }
            }
        }
    }
}