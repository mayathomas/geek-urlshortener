use anyhow::Result;
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite://urlshorten.db").await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS urls (
            id CHAR(6) PRIMARY KEY,
            url TEXT NOT NULL UNIQUE
        )
        "#,
    )
    .execute(&pool)
    .await?;
    Ok(())
}
