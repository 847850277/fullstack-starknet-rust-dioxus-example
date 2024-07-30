use std::{collections::HashMap, ops::Deref, sync::Mutex, time::Duration};
use std::str::FromStr;

use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use sqlx::sqlite::SqliteConnectOptions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub security: String,
    pub address: String,
}


pub static DB: Lazy<SqlitePool> = Lazy::new(|| {
    log::info!("Initing the DB Pool lazily ...");
    let pool = SqlitePoolOptions::new()
        .min_connections(1)
        .max_connections(3)
        .idle_timeout(Duration::from_secs(280))
        .connect_lazy("sqlite::memory:");
    match pool {
        Ok(pool) => {
            log::info!("DB Pool lazily inited.");
            pool
        }
        Err(err) => {
            log::error!("Unable to init the DB Pool: {}", err);
            panic!("Unable to init the DB Pool: {}", err)
        }
    }
});

pub async fn is_db_pool_ready() -> Result<SqlitePool, String> {
    log::info!("DB Pool idle={} size={}", DB.num_idle(), DB.size());
    let db_pool: &SqlitePool = DB.deref();
    let row = sqlx::query( r#"
                CREATE TABLE IF NOT EXISTS users (
                    "id" INTEGER PRIMARY KEY,
                    "security" VARCHAR(256) NOT NULL,
                    "address" VARCHAR(256) NOT NULL
                )
            "#,)
        .execute(db_pool)
        .await
        .map_err(|err| err.to_string());

    match row {
        Ok(row) => {
            log::info!("create table users success.");

            // insert rows to table users
            let row = sqlx::query( r#"
                INSERT INTO users (security, address) VALUES ('123456', '0x123456')
            "#,).execute(db_pool).await.map_err(|err| err.to_string());
            match row {
                Ok(row) => {
                    log::info!("insert rows to table users success.");
                },
                Err(err) => {
                    log::error!("insert rows to table users failed: {}", err);
                    return Err(err.to_string());
                }
            }

            Ok(db_pool.clone())
        },
        Err(err) => {
            log::error!("create table users failed: {}", err);
            Err(err)
        }
    }
}