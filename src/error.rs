#[cfg(feature = "server")]
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use tracing::info;

#[derive(Debug, Clone)]
pub enum Error {
    CouldNotCreateTicket,
    TicketNotFound,
    AuthFail,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "server")]
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut res = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        info!("{self}");

        res.extensions_mut().insert(self);

        res
    }
}

pub type Result<T> = core::result::Result<T, Error>;
