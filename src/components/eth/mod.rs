use dioxus::prelude::*;

use crate::services::eth::*;

#[component]
pub fn Eth() -> Element {
    //require_login!();

    // todo get

    // https://sepolia.voyager.online/api/contract/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7/functions

    let functions = vec![1,2,3,00,888];
    let address = String::new();
    //let contract_function = get_eth_write_fun(address);
    //println!("{} {}", "{}", contract_function);
    //println!("{}", contract_function);
    //TODO render write function
    println!("{}", 123);
    rsx! {
        div {
            class: "space-y-4 w-2/5",
            for (i , t) in functions.iter().enumerate(){
                div{
                    p {
                        class: "text",
                        "{t}"
                    }
                }

            }

        }
    }

}