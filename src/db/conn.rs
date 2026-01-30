use sqlx::sqlite::SqlitePool;
use std::env;

pub async fn init_db() -> anyhow::Result<SqlitePool> {
    dotenvy::dotenv().ok();
    let url = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;
    sqlx::migrate!("./migrations/").run(&pool).await?;
    Ok(pool)
}
