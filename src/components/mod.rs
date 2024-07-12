macro_rules! require_login {
    () => {
        let logged_in = consume_context::<Signal<bool>>();
        if !*logged_in.read()
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
