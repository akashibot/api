use crate::app::AppState;
use crate::util::errors::{bad_request, not_found, AppResult};
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GetUserTags {
    pub user_id: String,
    pub guild_id: String,
}

/// Retrieve an user's tags from a guild.
///
/// `GET /tags/user/list`
pub async fn metadata(
    app: AppState,
    _req: Parts,
    Valid(Json(tag_body)): Valid<Json<GetUserTags>>,
) -> AppResult<Json<Value>> {
    let db = app.database();

    let tags = db.get_user_tags(tag_body.user_id, tag_body.guild_id).await.map_err(|_| {
        not_found()
    })?;

    if tags.is_empty() {
        return Err(bad_request("no tags found for this user"));
    }

    Ok(Json(json!(tags)))
}
