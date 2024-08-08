use crate::services::login::*;

macro_rules! require_login {
    () => {
        let login_stats_result = use_resource(move || async move {
            get_login_data().await
        });
        let login = login_stats_result.read_unchecked().clone();
        let mut logged_in = false;
        match login {
            Some(Ok(value)) => {
                logged_in = value;
            }
            _ => {
                logged_in = false;
            }
        }

        if !logged_in
        {
            return rsx!(
                div {
                    class: "space-y-4 w-2/5",
                    h3 {
                        class: "text",
                        "Please log in to view this page"
                    }
                    p{}
                    a {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        href: "/login",
                        "click to Login"
                    }

                }
            );
        }
    };
}

pub mod home;
pub mod login;
pub mod navbar;
pub mod tickets;
pub mod page_not_found;
pub mod eth;

pub use home::*;
pub use login::*;
pub use navbar::*;
pub use tickets::*;
