use dioxus::prelude::*;

use crate::list_tickets;

#[component]
pub fn Tickets() -> Element {
    require_login!();

    let mut tickets = use_resource(move || async move {
        list_tickets()
            .await
            .unwrap_or("Failed to fetch tickets".to_string())
    });

    let ticketstr = match &*tickets.read() {
        Some(text) => text.clone(),
        _ => String::from(""),
    };

    rsx! {
        div {
            class: "space-y-4 w-2/5",
            p {
                class: "text",
                "{ticketstr}"
            }
        }
    }
}