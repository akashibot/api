use crate::database::operations::{create_guild, get_guild};
use crate::database::{GuildModel};
use sqlx::PgPool;

/// Provides access to a database using sqlx operations.
#[derive(Clone)]
pub struct PgDbClient {
    pool: PgPool,
}

impl PgDbClient {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_guild(&self, id: String) -> anyhow::Result<GuildModel> {
        create_guild(
            &self.pool,
            &GuildModel {
                id,
                plan: "free".to_string(),
                created_at: chrono::Utc::now().naive_utc(),
            },
        )
        .await
    }

    pub async fn get_guild(&self, id: &str) -> anyhow::Result<Option<GuildModel>> {
        get_guild(&self.pool, id).await
    }
}
