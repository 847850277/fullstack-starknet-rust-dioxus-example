use dioxus::html::link;
use dioxus::prelude::*;

use crate::services::login::login_page;
use crate::Route;

#[derive(PartialEq, Clone)]
struct Error<'a> {
    msg: &'a str,
}

#[component]
pub fn Login() -> Element {
    let mut logged_in = consume_context::<Signal<bool>>();
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_message = use_signal(|| "".to_string());
    let submit = move |_| async move {
        let res = login_page(username.to_string().clone(), password.to_string().clone()).await;
        match res {
            Ok(success) => {
                *logged_in.write() = success;
                //redirect
                let nav = navigator();
                nav.replace(Route::Contracts {});
            }

            Err(e) => {
                *logged_in.write() = false;
                error_message.set(e.to_string());
            }
        }
    };



    rsx! {

        div {
            class: "space-y-4 w-2/5",
            div{
                class: "space-y-4 justify-center",
                p{ "{error_message.read()}" } // Display the error message

            }
            div {
                class: "space-y-4 justify-center",
                input {
                    class: "block w-full p-2 border border-gray-300",
                    placeholder: "Enter address",
                    required: true,
                    oninput: move |text| username.set(text.value())
                }
                input {
                    class: "block w-full p-2 border border-gray-300",
                    placeholder: "Enter security",
                    required: true,
                    oninput: move |text| password.set(text.value())
                }
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: submit,
                    "Login"
                }
            }
        }
    }
}