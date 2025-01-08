use crate::app::AppState;
use crate::util::errors::{not_found, AppResult};
use axum::extract::Path;
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct DeleteGuild {
    pub id: String,
}

/// Handles the requests for deleting a Guild.
///
/// `DELETE /guilds/:id`
pub async fn delete(
    app: AppState,
    _req: Parts,
    Valid(Path(guild)): Valid<Path<DeleteGuild>>,
) -> AppResult<Json<Value>> {
    let db = app.database();

    db.delete_guild(guild.id).await.map_err(|_| not_found())?;

    Ok(Json(json!({
        "message": "guild deleted"
    })))
}
