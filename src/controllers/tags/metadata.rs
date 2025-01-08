use crate::app::AppState;
use crate::util::errors::{not_found, AppResult};
use axum::extract::Path;
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GetTagPath {
    pub guild_id: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GetTagBody {
    pub name: String,
}

/// Retrieves a tag from a guild.
///
/// `GET /tags/:guild_id`
pub async fn metadata(
    app: AppState,
    _req: Parts,
    Valid(Path(tag_path)): Valid<Path<GetTagPath>>,
    Valid(Json(tag_body)): Valid<Json<GetTagBody>>,
) -> AppResult<Json<Value>> {
    let db = app.database();

    let tag = db
        .get_tag(tag_path.guild_id, tag_body.name)
        .await
        .map_err(|_| not_found())?;

    if let Some(tag) = tag {
        return Ok(Json(json!(tag)));
    }

    Err(not_found())
}
