use sqlx::sqlite::SqlitePool;
use std::sync::{Arc, Mutex};

use crate::{error, Ticket};
use pwhash::bcrypt;

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug, sqlx::FromRow)]
pub struct SqlTicket {
    id: i64,
    title: String,
    body: String,
    user_id: i64,
}

impl From<SqlTicket> for Ticket {
    fn from(value: SqlTicket) -> Self {
        Self {
            id: value.id,
            title: value.title,
            body: value.body,
            user_id: value.user_id,
        }
    }
}

pub struct ModelController {
    pool: SqlitePool,
}

impl ModelController {
    pub async fn new(db: SqlitePool) -> Self {
        Self { pool: db }
    }

    pub async fn create_ticket_tables(&self) {
        let pool = &self.pool;
        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS tickets (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
                    "title" VARCHAR(256) NOT NULL,
                    "body" VARCHAR(256) NOT NULL,
                    "user_id" INTEGER,
                    FOREIGN KEY("user_id") REFERENCES users("id")
                )
            "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                INSERT INTO tickets
                    (id, title, body, user_id) SELECT 9, 'This is a test ticket content', 'This is a test ticket title', 2
            "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                DELETE FROM tickets WHERE "id" = 9
            "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                INSERT INTO tickets
                    (title, body, user_id) SELECT 'This is a test ticket content', 'This is a test ticket title', 2
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
    }

    pub async fn create_ticket(
        &self,
        title: String,
        body: String,
        userid: i64,
    ) -> anyhow::Result<SqlTicket> {
        let mut pool = self.pool.begin().await?;

        sqlx::query(
            r#"
                INSERT INTO tickets
                    (title, body, user_id) VALUES ($1, $2, $3)
            "#,
        )
        .bind(title)
        .bind(body)
        .bind(userid)
        .execute(&mut *pool)
        .await?;

        let ticket = sqlx::query_as::<_, SqlTicket>(
            r#"
                SELECT * FROM tickets where id = last_insert_rowid()
            "#,
        )
        .fetch_one(&mut *pool)
        .await?;

        pool.commit().await?;

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> anyhow::Result<Vec<SqlTicket>> {
        let pool = &self.pool;

        let tickets = sqlx::query_as::<_, SqlTicket>(
            r#"
                SELECT * FROM TICKETS
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(tickets)
    }

    pub async fn get_user(&self, username: String) -> anyhow::Result<crate::auth::User> {
        let pool = &self.pool;

        let (id,) = sqlx::query_as::<_, (i64,)>(
            r#"
                SELECT id FROM users WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        let user = crate::auth::User::get_user(id, pool)
            .await
            .ok_or(crate::error::Error::AuthFail)?;

        Ok(user)
    }

    pub async fn create_user(
        &self,
        username: String,
        pw: String,
    ) -> anyhow::Result<crate::auth::User> {
        let pool = &self.pool;

        let pwhash = bcrypt::hash(pw)?;

        sqlx::query(
            r#"
                INSERT INTO users 
                    (anonymous, username, pwhash) values ($1, $2, $3)
            "#,
        )
        .bind(false)
        .bind(username.clone())
        .bind(pwhash.clone())
        .execute(pool)
        .await?;

        let user = self.get_user(username).await?;

        sqlx::query(
            r#"
                INSERT INTO user_permissions
                    (user_id, token) VALUES ($1, $2)
            "#,
        )
        .bind(user.id)
        .bind("Category::View")
        .execute(pool)
        .await?;

        Ok(user)
    }
}
