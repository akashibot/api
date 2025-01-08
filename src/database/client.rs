use crate::database::operations::{
    create_guild, create_tag, create_user, delete_guild, get_guild, get_tag, get_user,
    get_user_tags, update_guild,
};
use crate::database::{GuildModel, TagModel, UserModel};
use sqlx::types::uuid;
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
                commands_ran: Some(0),
                created_at: Some(chrono::Utc::now().naive_utc()),
            },
        )
        .await
    }

    pub async fn get_guild(&self, id: String) -> anyhow::Result<Option<GuildModel>> {
        get_guild(&self.pool, id).await
    }

    pub async fn update_guild(&self, id: String) -> anyhow::Result<GuildModel> {
        update_guild(&self.pool, id).await
    }

    pub async fn delete_guild(&self, id: String) -> anyhow::Result<()> {
        delete_guild(&self.pool, id).await
    }

    pub async fn create_user(&self, id: String) -> anyhow::Result<UserModel> {
        create_user(
            &self.pool,
            &UserModel {
                id,
                created_at: Some(chrono::Utc::now().naive_utc()),
            },
        )
        .await
    }

    pub async fn get_user(&self, id: String) -> anyhow::Result<Option<UserModel>> {
        get_user(&self.pool, id).await
    }

    pub async fn get_user_tags(
        &self,
        id: String,
        guild_id: String,
    ) -> anyhow::Result<Vec<TagModel>> {
        get_user_tags(&self.pool, id, guild_id).await
    }

    pub async fn create_tag(
        &self,
        guild_id: String,
        name: String,
        content: String,
        created_by: String,
    ) -> anyhow::Result<TagModel> {
        create_tag(
            &self.pool,
            &TagModel {
                id: uuid::Uuid::new_v4().to_string(),
                guild_id: Some(guild_id),
                name,
                content,
                created_by: Some(created_by),
                created_at: Some(chrono::Utc::now().naive_utc()),
            },
        )
        .await
    }

    pub async fn get_tag(
        &self,
        guild_id: String,
        name: String,
    ) -> anyhow::Result<Option<TagModel>> {
        get_tag(&self.pool, guild_id, name).await
    }
}
