use dioxus::html::map;
use dioxus::prelude::*;
use serde::Deserialize;
use crate::route::Route;

use crate::services::eth::*;
use crate::services::login::*;
use std::collections::HashMap;

#[component]
pub fn Eth() -> Element {
    require_login!();

    let mut contracts_funcs = use_resource(move || async move {
        get_server_data().await
    });
    let contracts = contracts_funcs.read_unchecked().clone();
    match contracts {
        // two tab for functions and state changing functions
        Some(Ok(value)) => {
            let functions = value.functions.clone();
            let copy_functions = use_signal(|| functions.clone());
            let contract_address = use_signal(|| value.address.clone());
            //let mut error_messages = use_signal(|| Vec::<String>::new());
            let mut error_messages = use_signal(|| vec!["".to_string(); functions.len()]);
            let my_state_changing_functions = value.state_changing_functions.clone().unwrap_or_default();
            let my_state_changing_functions_copy = use_signal(|| my_state_changing_functions.clone());
            let mut show_parameters = use_signal(|| vec![HashMap::<String, String>::new(); my_state_changing_functions.len()]);
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
                                                    let contract_address = contract_address.read().clone();
                                                    let response = call_read_function(clone.name,contract_address).await;
                                                    match response {
                                                        Ok(value) => {
                                                            //error_message.set(value.to_string());
                                                            let mut array = error_messages.read().clone();
                                                            // index push
                                                            array[index] = value.to_string();
                                                            error_messages.set(array);
                                                        },
                                                        Err(e) => {
                                                            // Display the error message
                                                            let mut array = error_messages.read().clone();
                                                            // index push
                                                            array[index] = e.to_string();
                                                            error_messages.set(array);
                                                        }
                                                    }
                                                },
                                                "{func.name}"
                                            }
                                            p{ "{error_messages.read()[index]}" } // Display the error message
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
                            hr{}
                            if let Some(state_changing_functions) = &value.state_changing_functions {
                                for (index, func) in state_changing_functions.iter().enumerate() {
                                    li {
                                        h3{
                                            "{func.name}"
                                        }
                                        for (i,parameter) in func.parameters.iter().enumerate() {
                                            div{
                                                    class: "space-y-4 justify-center ",
                                                    br{},
                                                    input {
                                                        class: "block w-full p-2 border border-gray-300",
                                                        name: "{parameter.name}",
                                                        placeholder: "Enter {parameter.name} value",
                                                        oninput: move |text| {
                                                            let mut array = show_parameters.read().clone();
                                                            // index push
                                                            let my_state_changing_functions_copy = my_state_changing_functions_copy.read().clone();
                                                            let key = my_state_changing_functions_copy[index].parameters[i].name.clone();
                                                            let value = text.value();
                                                            array[index].insert(key, value);
                                                            show_parameters.set(array);
                                                        }
                                                    },
                                                    // i is last
                                                    if i == func.parameters.len() - 1 {
                                                        br {}
                                                        button {
                                                            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                                                            onclick: move |_| async move{
                                                                let clone = my_state_changing_functions_copy.read()[index].clone();
                                                                let contract_address = contract_address.read().clone();
                                                                let param =  show_parameters.read().get(index).unwrap().clone();
                                                                let response = call_write_function(clone.name,contract_address,param).await;
                                                                match response {
                                                                    Ok(value) => {
                                                                        //error_message.set(value.to_string());
                                                                        let mut array = error_messages.read().clone();
                                                                        // index push
                                                                        array[index] = value.to_string();
                                                                        error_messages.set(array);
                                                                    },
                                                                    Err(e) => {
                                                                        // Display the error message
                                                                        let mut array = error_messages.read().clone();
                                                                        // index push
                                                                        array[index] = e.to_string();
                                                                        error_messages.set(array);
                                                                    }
                                                                }
                                                            },
                                                            "call write function"
                                                        }
                                                    }
                                            }
                                        }
                                        hr{}
                                        p {
                                            for (key, value) in show_parameters.read()[index].iter() {
                                                "{key}: {value}"
                                            }
                                        }
                                    }
                                }
                            } else {
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



