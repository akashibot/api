//! Provides access to the database.
use chrono::NaiveDateTime;
pub use client::PgDbClient;

mod client;
pub(crate) mod operations;

/// Snowflake ID.
type Snowflake = String;

// Represents a guild in the database.
#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "guilds")]
pub struct GuildModel {
    pub id: Snowflake,
    // pub name: String,
    pub commands_ran: Option<i64>,
    pub created_at: Option<NaiveDateTime>,
}

// Represents a user in the database.
#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "users")]
pub struct UserModel {
    pub id: Snowflake,
    pub created_at: Option<NaiveDateTime>,
}

// Represents a tag in the database.
#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tags")]
pub struct TagModel {
    pub id: String,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    pub content: String,
    pub created_by: Option<Snowflake>,
    pub created_at: Option<NaiveDateTime>,
}
