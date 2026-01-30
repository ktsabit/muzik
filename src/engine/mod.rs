use sqlx::SqlitePool;
use std::path::PathBuf;

use crate::domain::Root;

pub struct MuzikEngine {
    #[allow(dead_code)]
    pool: SqlitePool,
}

impl MuzikEngine {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_root(&self, path: PathBuf) -> anyhow::Result<i64> {
        let absolute_path = std::fs::canonicalize(&path)
            .map_err(|e| anyhow::anyhow!("Failed to clarify path '{:?}': {}", path, e))?;
        let id = crate::db::root::add_root(&self.pool, absolute_path).await?;
        Ok(id)
    }

    pub async fn list_roots(&self) -> anyhow::Result<Vec<Root>> {
        let roots = crate::db::root::get_roots(&self.pool).await?;
        Ok(roots)
    }

    pub async fn remove_root(&self, id: i64) -> anyhow::Result<()> {
        crate::db::root::remove_root(&self.pool, id).await?;
        Ok(())
    }
}
