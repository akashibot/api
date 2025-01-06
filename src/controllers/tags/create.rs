use crate::app::AppState;
use crate::util::errors::{bad_request, AppResult};
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTag {
    pub guild_id: String,
    pub name: String,
    pub user_id: String,
    pub content: String
}

/// Creates a new tag.
///
/// `POST /tags`
pub async fn create(
    app: AppState,
    _req: Parts,
    Valid(Json(tag)): Valid<Json<CreateTag>>,
) -> AppResult<Json<Value>> {
    let db = app.database();

    let tag = db
        .create_tag(
            tag.guild_id,
            tag.name,
            tag.content,
            tag.user_id
        )
        .await
        .map_err(|_| bad_request("tag already exists in this guild"))?;

    Ok(Json(json!(tag)))
}
