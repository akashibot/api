//! Provides access to the database.
use chrono::NaiveDateTime;
pub use client::PgDbClient;
use sqlx::types::BigDecimal;

mod client;
pub(crate) mod operations;

/// Snowflake ID.
type Snowflake = String;

// Represents a guild in the database.
#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "guilds")]
pub struct GuildModel {
    pub id: Snowflake,
    pub plan: String,
    pub created_at: NaiveDateTime,
}
