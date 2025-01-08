use crate::app::AppState;
use crate::util::errors::{not_found, AppResult};
use axum::extract::Path;
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GetGuild {
    pub id: String,
}

/// Handles the requests for retrieving a Guild.
///
/// `GET /guilds/:id`
pub async fn metadata(
    app: AppState,
    _req: Parts,
    Valid(Path(guild)): Valid<Path<GetGuild>>,
) -> AppResult<Json<Value>> {
    let db = app.database();

    let guild = db.get_guild(guild.id).await.map_err(|_| not_found())?;

    if let Some(guild) = guild {
        return Ok(Json(json!(guild)));
    }

    Err(not_found())
}
