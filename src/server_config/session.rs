use axum::{async_trait, http};
use axum_session_sqlx::SessionSqlitePool;
use tracing::info;
use crate::server_config::start::ServerState;

pub struct Session {
    /// The database connection pool.
    pub dbp: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>,
    pub axum_session: axum_session::Session<SessionSqlitePool>
}

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for Session
where
    S: std::marker::Sync + std::marker::Send,
{
    type Rejection = StateError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        //
        let ss = parts.extensions.get::<ServerState>().unwrap();
        let dbp = ss.0.clone();
        let axum_session = (*parts.extensions.get::<axum_session::Session<SessionSqlitePool>>().unwrap()).clone();
        log::debug!("parts: {:?}", parts);
        Ok(Session { dbp,axum_session })
    }
}

#[derive(Debug)]
pub struct StateError;

impl std::error::Error for StateError {}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(internal) state error")
    }
}

impl axum::response::IntoResponse for StateError {
    fn into_response(self) -> axum::response::Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "(internal) state error",
        )
            .into_response()
    }
}