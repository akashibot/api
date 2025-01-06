use super::{GuildModel, TagModel, UserModel};
use sqlx::PgExecutor;

// TODO: Check queries

/// Creates a guild in the database.
pub(crate) async fn create_guild(
    executor: impl PgExecutor<'_>,
    data: &GuildModel,
) -> anyhow::Result<GuildModel> {
    let guild = sqlx::query_as!(
        GuildModel,
        r#"
        INSERT INTO guilds (id, created_at, commands_ran)
        VALUES ($1, $2, $3)
        RETURNING id, commands_ran, created_at
        "#,
        data.id,
        data.created_at,
        data.commands_ran
    )
    .fetch_one(executor)
    .await?;

    Ok(guild)
}

/// Updates a guild in the database.
pub(crate) async fn update_guild(
    executor: impl PgExecutor<'_>,
    id: String
) -> anyhow::Result<GuildModel> {
    let guild = sqlx::query_as!(
        GuildModel,
        r#"
        UPDATE guilds
        SET commands_ran = $2
        WHERE id = $1
        RETURNING id, commands_ran, created_at
        "#,
        id,
        0
    )
    .fetch_one(executor)
    .await?;

    Ok(guild)
}

/// Retrieves a guild from the database.
pub(crate) async fn get_guild(
    executor: impl PgExecutor<'_>,
    id: String,
) -> anyhow::Result<Option<GuildModel>> {
    let guild = sqlx::query_as!(
        GuildModel,
        r#"
        SELECT id, created_at, commands_ran
        FROM guilds
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(executor)
    .await?;

    Ok(guild)
}

/// Deletes a guild from the database.
pub(crate) async fn delete_guild(
    executor: impl PgExecutor<'_>,
    id: String,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM guilds
        WHERE id = $1
        "#,
        id
    )
    .execute(executor)
    .await?;

    Ok(())
}

/// Creates a user in the database.
pub(crate) async fn create_user(
    executor: impl PgExecutor<'_>,
    data: &UserModel,
) -> anyhow::Result<UserModel> {
    let user = sqlx::query_as!(
        UserModel,
        r#"
        INSERT INTO users (id, created_at)
        VALUES ($1, $2)
        RETURNING id, created_at
        "#,
        data.id,
        data.created_at
    )
    .fetch_one(executor)
    .await?;

    Ok(user)
}

/// Retrieves an user from the database.
pub(crate) async fn get_user(
    executor: impl PgExecutor<'_>,
    id: String,
) -> anyhow::Result<Option<UserModel>> {
    let user = sqlx::query_as!(
        UserModel,
        r#"
        SELECT id, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(executor)
    .await?;

    Ok(user)
}

/// Retrieves the tags an user has created in the provided guild.
pub(crate) async fn get_user_tags(
    executor: impl PgExecutor<'_>,
    id: String,
    guild_id: String,
) -> anyhow::Result<Vec<TagModel>> {
    let tags = sqlx::query_as!(
        TagModel,
        r#"
        SELECT id, guild_id, name, content, created_by, created_at
        FROM tags
        WHERE created_by = $1 AND guild_id = $2
        "#,
        id,
        guild_id
    )
    .fetch_all(executor)
    .await?;

    Ok(tags)
}

/// Creates a tag in the database.
pub(crate) async fn create_tag(
    executor: impl PgExecutor<'_>,
    data: &TagModel,
) -> anyhow::Result<TagModel> {
    let tag = sqlx::query_as!(
        TagModel,
        r#"
        INSERT INTO tags (guild_id, name, content, created_by, created_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, guild_id, name, content, created_by, created_at
        "#,
        data.guild_id,
        data.name,
        data.content,
        data.created_by,
        data.created_at
    )
    .fetch_one(executor)
    .await?;

    Ok(tag)
}

/// Retrieves a tag from the database.
pub(crate) async fn get_tag(
    executor: impl PgExecutor<'_>,
    guild_id: String,
    name: String,
) -> anyhow::Result<Option<TagModel>> {
    let tag = sqlx::query_as!(
        TagModel,
        r#"
        SELECT id, guild_id, name, content, created_by, created_at
        FROM tags
        WHERE guild_id = $1 AND name = $2
        "#,
        guild_id,
        name
    )
    .fetch_optional(executor)
    .await?;

    Ok(tag)
}