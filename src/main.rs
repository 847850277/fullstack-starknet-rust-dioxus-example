#![allow(non_snake_case, unused)]

use std::sync::{Arc, Mutex};

use dioxus::prelude::{
    server_fn::middleware::{self, Layer},
    *,
};
use tokio::sync::OnceCell;
use tracing::{info, Level};

#[cfg(feature = "server")]
use axum_session_auth::*;

#[cfg(feature = "server")]
mod auth;

mod error;

#[cfg(feature = "server")]
mod model;

#[cfg(feature = "server")]
use model::*;

#[cfg(feature = "server")]
use pwhash::bcrypt;
use tracing::metadata::LevelFilter;

#[cfg(feature = "server")]
static STATE: OnceCell<ModelController> = OnceCell::const_new();

mod components;
mod route;
mod services;

use components::{Home, Login, NavBar, Tickets};

use route::Route;

macro_rules! require {
    ($method:expr, $user:expr, $msg:expr, $perms:expr) => {
        if !axum_session_auth::Auth::<crate::auth::User, i64, sqlx::SqlitePool>::build(
            [axum::http::Method::POST],
            false,
        )
        .requires($perms)
        .validate(&$user, &$method, None)
        .await
        {
            return Ok($msg);
        }
    };
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct Ticket {
    id: i64,
    title: String,
    body: String,
    user_id: i64,
}

impl std::fmt::Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.title, self.body)
    }
}

fn main() {

    //dioxus_logger::init(Level::DEBUG).expect("failed to init logger");


    #[cfg(feature = "web")]
    // Hydrate the application on the client
    dioxus::web::launch::launch_cfg(App, dioxus::web::Config::new().hydrate(true));

    #[cfg(feature = "server")]
    {
        use crate::auth::*;
        use axum::routing::*;
        use axum_session::SessionConfig;
        use axum_session::SessionStore;
        use axum_session_auth::AuthConfig;
        use axum_session_sqlx::SessionSqlitePool;
        dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let pool = connect_to_database().await;

                //This Defaults as normal Cookies.
                //To enable Private cookies for integrity, and authenticity please check the next Example.
                let session_config = SessionConfig::default().with_table_name("test_table");
                let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
                let session_store = SessionStore::<SessionSqlitePool>::new(
                    Some(pool.clone().into()),
                    session_config,
                )
                .await
                .unwrap();

                User::create_user_tables(&pool).await;

                STATE
                    .get_or_init(|| async { ModelController::new(pool.clone()).await })
                    .await;

                STATE.get().unwrap().create_ticket_tables().await;

                // build our application with some routes
                let app = Router::new()
                    // Server side render the application, serve static assets, and register server functions
                    .serve_dioxus_application(ServeConfig::builder().build(), || {
                        VirtualDom::new(App)
                    })
                    .await
                    .layer(
                        axum_session_auth::AuthSessionLayer::<
                            crate::auth::User,
                            i64,
                            SessionSqlitePool,
                            sqlx::SqlitePool,
                        >::new(Some(pool))
                        .with_config(auth_config),
                    )
                    .layer(axum_session::SessionLayer::new(session_store));

                // run it
                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }
}

fn App() -> Element {
    use_context_provider(|| Signal::new(false));
    rsx! {
        div {
            class: "dark flex justify-center",
            Router::<Route> {}
        }
    }
}



#[server(CreateTicket)]
pub async fn create_ticket(title: String, body: String) -> Result<Ticket, ServerFnError> {
    let method: axum::http::Method = extract().await?;
    let auth: crate::auth::Session = extract().await?;
    let current_user = auth.current_user.clone().unwrap_or_default();

    require!(
        method,
        current_user,
        Ticket {
            title: "Log in please".into(),
            body: "Log in please".into(),
            id: 0,
            user_id: 0
        },
        axum_session_auth::Rights::any([
            axum_session_auth::Rights::permission("Category::View"),
            axum_session_auth::Rights::permission("Admin::View"),
        ])
    );

    Ok(STATE
        .get()
        .expect("Failed to connect to database")
        .create_ticket(title, body, current_user.id.into())
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::new("Problem creating ticket")
        })?
        .into())
}

#[server]
pub async fn list_tickets() -> Result<String, ServerFnError> {
    let method: axum::http::Method = extract().await?;
    let auth: crate::auth::Session = extract().await?;
    let current_user = auth.current_user.clone().unwrap_or_default();

    require!(
        method,
        current_user,
        format!(
            "User {} does not have the permissions to view this page. please login",
            current_user.username
        ),
        axum_session_auth::Rights::any([
            axum_session_auth::Rights::permission("Category::View"),
            axum_session_auth::Rights::permission("Admin::View"),
        ])
    );

    Ok(STATE
        .get()
        .expect("Could not connect to db")
        .list_tickets()
        .await
        .map_err(|_| ServerFnError::new("Could not communicate with db"))?
        .into_iter()
        .map(|sqltick| Ticket::from(sqltick))
        .map(|ticket| format!("{ticket}"))
        .collect())
}

#[server]
pub async fn login(username: String, password: String) -> Result<i64, ServerFnError> {

    // let dbuser = STATE
    //     .get()
    //     .expect("Could not connect to db")
    //     .get_user(username.clone())
    //     .await
    //     .map_err(|_| ServerFnError::new("Auth fail"))?;
    //
    // if username != dbuser.username {
    //     return Err(ServerFnError::new("Auth fail"));
    // }
    //
    // if !bcrypt::verify(password, &dbuser.pwhash) {
    //     return Err(ServerFnError::new("Auth fail"));
    // }

    Ok(1)
}
