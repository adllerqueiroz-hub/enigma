use super::CollectionManager;
use crate::error::AppError;
use database::db::game::handbook;
use sonettobuf::{GetHandbookInfoReply, HandbookReadReply};
use sqlx::SqlitePool;

impl CollectionManager {
    pub async fn handbook_info(&self, db: &SqlitePool) -> Result<GetHandbookInfoReply, AppError> {
        Ok(GetHandbookInfoReply {
            infos: handbook::get_handbook_reads(db, self.player_id).await?,
            element_info: handbook::get_handbook_fragments(db, self.player_id).await?,
        })
    }

    pub async fn read_handbook(
        &self,
        db: &SqlitePool,
        r#type: i32,
        id: i32,
    ) -> Result<HandbookReadReply, AppError> {
        if !(1..=4).contains(&r#type) || id <= 0 {
            return Err(AppError::InvalidRequest);
        }
        handbook::mark_read(db, self.player_id, r#type, id).await?;
        Ok(HandbookReadReply {
            r#type: Some(r#type),
            id: Some(id),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handbook_read_persists_known_type() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        database::run_migrations(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO users (id, username, created_at, updated_at) VALUES (11, 'book', 0, 0)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let manager = CollectionManager::new(11);
        manager.read_handbook(&pool, 3, 3125).await.unwrap();
        assert!(manager.read_handbook(&pool, 5, 3125).await.is_err());
    }
}
