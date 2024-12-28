use super::{GuildModel};
use sqlx::{types::BigDecimal, PgExecutor, PgPool};

// TODO: Check queries

/// Creates or updates a guild in the database.
pub(crate) async fn create_guild(
    executor: impl PgExecutor<'_>,
    data: &GuildModel,
) -> anyhow::Result<GuildModel> {
    // language=PostgreSQL
    let guild = sqlx::query_as!(
        GuildModel,
        r#"
        INSERT INTO guilds (id, plan, created_at)
        VALUES ($1, $2, $3)
        ON CONFLICT (id) DO UPDATE
        SET plan = $2
        RETURNING id, plan, created_at
        "#,
        data.id,
        data.plan,
        data.created_at
    )
    .fetch_one(executor)
    .await?;

    Ok(guild)
}

/// Retrieves a guild from the database.
pub(crate) async fn get_guild(
    executor: impl PgExecutor<'_>,
    id: &str,
) -> anyhow::Result<Option<GuildModel>> {
    // language=PostgreSQL
    let guild: Option<GuildModel> = sqlx::query_as!(
        GuildModel,
        r#"
        SELECT id, plan, created_at
        FROM guilds
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(executor)
    .await?;

    Ok(guild)
}