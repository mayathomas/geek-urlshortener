use std::ops::Deref;

use anyhow::Result;
use futures::future::BoxFuture;
use nanoid::nanoid;
use sqlx::MySqlPool;
use tracing::{debug, error, warn};

use crate::{error::AppError, model::UrlRecord};

const MYSQL_DUPLICATE_KEY_ERROR: &str = "23000";

impl UrlRecord {
    pub fn shorten(
        id: String,
        url: String,
        pool: MySqlPool,
    ) -> BoxFuture<'static, Result<String, AppError>> {
        Box::pin(async move {
            // let id = nanoid!(50);
            let ret = sqlx::query("INSERT INTO urls (id, url) VALUES (?, ?)")
                .bind(&id)
                .bind(url.clone())
                .execute(&pool)
                .await;
            let id = match ret {
                Ok(ret) => {
                    debug!("insert id: {}", ret.last_insert_id());
                    id
                }
                Err(e) => {
                    // if meet the duplicate error, call this shorten function recursively
                    if let sqlx::Error::Database(e) = e {
                        let code = e.deref().code().unwrap();
                        if code == MYSQL_DUPLICATE_KEY_ERROR {
                            warn!("Duplicate id: {}", id);
                            let id = nanoid!(10);
                            UrlRecord::shorten(id, url, pool).await?
                        } else {
                            error!("{:?}", e);
                            return Err(AppError::Error("Unknown sql database error".to_string()));
                        }
                    } else {
                        error!("{:?}", e);
                        return Err(AppError::Error("Unknown sql error".to_string()));
                    }
                }
            };
            Ok(id)
        })
    }

    pub async fn get_by_id(id: String, pool: MySqlPool) -> Result<String, AppError> {
        let ret: Result<UrlRecord, sqlx::Error> = sqlx::query_as("SELECT * FROM urls WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await;
        let ret = match ret {
            Ok(ret) => ret.url,
            Err(e) => {
                error!("{:?}", e);
                return Err(AppError::Error("Unknown sql error".to_string()));
            }
        };
        Ok(ret)
    }
}
