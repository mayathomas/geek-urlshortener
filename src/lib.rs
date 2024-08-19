use std::sync::Arc;

use sqlx::MySqlPool;

use anyhow::Result;
mod error;
mod handler;
mod model;
mod service;
pub use handler::{redirect, url_shortener};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub protocol: Arc<Protocol>,
    pub addr: Arc<String>,
}

#[derive(Debug, Copy, Clone)]
pub enum Protocol {
    HTTP,
    HTTPS,
}

impl From<Protocol> for String {
    fn from(value: Protocol) -> Self {
        match value {
            Protocol::HTTP => "http://".to_string(),
            Protocol::HTTPS => "https://".to_string(),
        }
    }
}

impl AppState {
    pub async fn try_new() -> Result<Self> {
        let pool = MySqlPool::connect("mysql://root:sorrow@localhost:3306/shortener").await?;
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS urls (
                id VARCHAR(50) PRIMARY KEY,
                url TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;
        Ok(Self {
            pool,
            protocol: Arc::new(Protocol::HTTP),
            addr: Arc::new("127.0.0.1:8080".to_string()),
        })
    }
}
