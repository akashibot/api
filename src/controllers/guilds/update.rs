use crate::app::AppState;
use crate::util::errors::{not_found, AppResult};
use axum::extract::Path;
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGuildPath {
    pub id: String,
}

/// Handles the requests for updating a Guild.
///
/// `PATCH /guilds/:id`
pub async fn update(
    app: AppState,
    _req: Parts,
    Valid(Path(guild_path)): Valid<Path<UpdateGuildPath>>,
) -> AppResult<Json<Value>> {
    let db = app.database();

    let guild = db
        .update_guild(guild_path.id)
        .await
        .map_err(|_| not_found())?;

    Ok(Json(json!(guild)))
}
