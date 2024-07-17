use dioxus::prelude::*;
use serde::Deserialize;

use crate::services::eth::*;

#[component]
pub fn Eth() -> Element {

    // let mut count = use_signal(|| 0);
    // let mut text = use_signal(|| "...".to_string());
    // let server_future = use_server_future(get_server_data)?;
    //
    // rsx! {
    //     h1 { "High-Five counter: {count}" }
    //     button { onclick: move |_| count += 1, "Up high!" }
    //     button { onclick: move |_| count -= 1, "Down low!" }
    //     button {
    //         onclick: move |_| async move {
    //             if let Ok(data) = get_server_data().await {
    //                 println!("Client received: {}", data);
    //                 text.set(data.clone());
    //                 post_server_data(data).await.unwrap();
    //             }
    //         },
    //         "Run a server function!"
    //     }
    //     "Server said: {text}"
    // }

    let mut resource = use_resource(move || async move {
        // This will run every time the revision signal changes because we read the count inside the future
        //reqwest::get("https://github.com/DioxusLabs/awesome-dioxus/blob/1d03b42/awesome.json").await
        get_server_data().await
    });
    match &*resource.read_unchecked() {
        Some(Ok(value)) => rsx! { "{value:?}" },
        Some(Err(err)) => rsx! { "Error: {err}" },
        None => rsx! { "Loading..." },
    }

}

#[server]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok(reqwest::get("https://httpbin.org/ip").await?.text().await?)
}