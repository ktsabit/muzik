use std::path::PathBuf;

use crate::domain::Root;
use sqlx::SqlitePool;

pub async fn get_roots(pool: &SqlitePool) -> anyhow::Result<Vec<Root>> {
    let roots = sqlx::query_as!(Root, "SELECT id, path FROM roots")
        .fetch_all(pool)
        .await?;
    Ok(roots)
}

pub async fn get_root(pool: &SqlitePool, id: i64) -> anyhow::Result<Root> {
    let root = sqlx::query_as!(Root, "SELECT id, path FROM roots WHERE id = ?1", id)
        .fetch_one(pool)
        .await?;
    Ok(root)
}

pub async fn add_root(pool: &SqlitePool, path: PathBuf) -> anyhow::Result<i64> {

    let path_str = path.to_string_lossy().to_string();

    let id = sqlx::query!(
        r#"INSERT INTO roots ( path ) VALUES ( ?1 )"#,
        path_str
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn remove_root(pool: &SqlitePool, id: i64) -> anyhow::Result<()> {
    sqlx::query!(r#"DELETE FROM roots WHERE id = ?1"#, id).execute(pool).await?;
    Ok(())
}


