use crate::app::AppState;
use crate::database::{GuildModel};
use crate::util::errors::{AppResult, not_found};
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use std::sync::Arc;
use axum::extract::Path;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GetGuild {
    pub id: String,
}

/// Handles the requests for retrieving a Guild.
/// `GET /guilds/:id`
pub async fn metadata(
    app: AppState,
    _req: Parts,
    Valid(Path(guild)): Valid<Path<GetGuild>>,
) -> AppResult<Json<Value>> {
    let db = Arc::clone(&app.0.db);

    let guild = db.get_guild(&guild.id).await?;

    if let Some(guild) = guild {
        return Ok(Json(json!(guild)));
    }

    Err(not_found())
}
