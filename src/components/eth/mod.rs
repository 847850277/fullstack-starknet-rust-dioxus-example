use dioxus::prelude::*;

use crate::services::eth::*;

#[component]
pub fn Eth() -> Element {

    //let mut revision = use_signal(|| "1d03b42");
    let mut resource = use_resource(move || async move {
        // This will run every time the revision signal changes because we read the count inside the future
        reqwest::get("https://github.com/DioxusLabs/awesome-dioxus/blob/1d03b42/awesome.json").await
    });
    match &*resource.read_unchecked() {
        Some(Ok(value)) => rsx! { "{value:?}" },
        Some(Err(err)) => rsx! { "Error: {err}" },
        None => rsx! { "Loading..." },
    }


    // rsx! {
    //     div {
    //         class: "space-y-4 w-2/5",
    //         for (i , t) in functions.iter().enumerate(){
    //             div{
    //                 p {
    //                     class: "text",
    //                     "{t}"
    //                 }
    //             }
    //
    //         }
    //
    //
    //     }
    //     div{
    //         p{
    //             class: "text",
    //                     "{name}"
    //         }
    //     }
    // }

}