#[cfg(feature = "server")]
use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, State},
};

#[cfg(feature = "server")]
use http::{request::Parts, Request};

#[cfg(feature = "server")]
#[cfg(feature = "server")]
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

#[cfg(feature = "server")]
use axum_session_auth::*;

use crate::AppState;

#[derive(Debug, Clone)]
pub(crate) struct User {
    pub username: String,
    pub id: i32,
    pub anonymous: bool,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "Guest".to_string(),
            id: 1,
            anonymous: true,
        }
    }
}

impl User {
    pub fn is_logged_in(&self) -> bool {
        !self.anonymous
    }
}

#[derive(Clone, Debug)]
pub struct MySession {
    pub user_id: i32,
}

#[cfg(feature = "server")]
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for MySession {
    type Rejection = crate::error::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> crate::error::Result<MySession> {
        parts
            .extensions
            .get::<crate::error::Result<MySession>>()
            .ok_or(crate::error::Error::AuthFail)?
            .clone()
    }
}

#[cfg(feature = "server")]
#[async_trait]
impl Authentication<User, i64, AppState> for User {
    async fn load_user(userid: i64, pool: Option<&AppState>) -> anyhow::Result<User> {
        let state = pool.ok_or(crate::error::Error::AuthFail)?;

        let user = state
            .inner
            .lock()
            .unwrap()
            .1
            .get(userid as usize)
            .and_then(|u| Some(u.clone()))
            .ok_or(crate::error::Error::AuthFail)?;
        Ok(user)
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}
