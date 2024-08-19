use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UrlRecord {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenReq {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenRes {
    pub url: String,
}
