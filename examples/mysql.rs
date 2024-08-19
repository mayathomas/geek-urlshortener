use anyhow::Result;
use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> Result<()> {
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
    Ok(())
}
