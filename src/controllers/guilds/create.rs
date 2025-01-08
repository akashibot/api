use crate::app::AppState;
use crate::util::errors::{bad_request, AppResult};
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGuild {
    pub id: String,
}

/// Handles the requests for creating a Guild.
///
/// `POST /guilds`
pub async fn create(
    app: AppState,
    _req: Parts,
    Valid(Json(guild)): Valid<Json<CreateGuild>>,
) -> AppResult<Json<Value>> {
    let db = app.database();

    let guild = db
        .create_guild(guild.id)
        .await
        .map_err(|_| bad_request("guild already exists"))?;

    Ok(Json(json!(guild)))
}
