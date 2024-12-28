use crate::app::AppState;
use crate::database::{GuildModel};
use crate::util::errors::AppResult;
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use std::sync::Arc;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGuild {
    pub id: String,
}

/// Handles the requests for creating a Guild.
/// `POST /guilds`
pub async fn create(
    app: AppState,
    _req: Parts,
    Valid(Json(guild)): Valid<Json<CreateGuild>>,
) -> AppResult<Json<Value>> {
    let db = Arc::clone(&app.0.db);

    db.create_guild(guild.id).await?;

    Ok(Json(json!({
        "message": "entry created"
    })))
}
