use std::sync::Arc;
use axum::Extension;
use crate::server::logging::init_logging;

use super::database::is_db_pool_ready;

use dioxus::dioxus_core::Element;

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct ServerState(
    /// The database connection pool.
    pub std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>,
);

pub fn start(app_fn: fn() -> Element) {
    init_logging();

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            let result = is_db_pool_ready().await;
            match result {
                Ok(db_pool) => {
                    log::info!("DB Pool is ready.");

                    use axum::Router;
                    use dioxus::prelude::*;

                    let state = ServerState(Arc::new(db_pool));
                    let web_api_router: Router<()> = Router::new()
                        // Server side render the application, serve static assets, and register server functions.
                        .serve_dioxus_application(ServeConfig::builder().build(), move || {
                            VirtualDom::new(app_fn)
                        })
                        .await
                        .layer(Extension(state));

                    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
                    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                    axum::serve(listener, web_api_router.into_make_service())
                        .await
                        .unwrap();

                },
                Err(err) => {
                    log::error!("DB Pool is not ready: {}", err);
                    return;
                }
            }
        });
}