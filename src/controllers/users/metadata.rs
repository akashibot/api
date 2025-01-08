use crate::app::AppState;
use crate::util::errors::{not_found, AppResult};
use axum::extract::Path;
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GetUser {
    pub id: String,
}

/// Handles the requests for retrieving a User.
///
/// `GET /users/:id`
pub async fn metadata(
    app: AppState,
    _req: Parts,
    Valid(Path(user)): Valid<Path<GetUser>>,
) -> AppResult<Json<Value>> {
    let db = app.database();

    let user = db.get_user(user.id).await.map_err(|_| not_found())?;

    if let Some(user) = user {
        return Ok(Json(json!(user)));
    }

    Err(not_found())
}
