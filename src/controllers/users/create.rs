use crate::app::AppState;
use crate::util::errors::{bad_request, AppResult};
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    pub id: String,
}

/// Handles the requests for creating an User.
///
/// `POST /users`
pub async fn create(
    app: AppState,
    _req: Parts,
    Valid(Json(user)): Valid<Json<CreateUser>>,
) -> AppResult<Json<Value>> {
    let db = app.database();

    let user = db
        .create_user(user.id)
        .await
        .map_err(|_| bad_request("user already exists"))?;

    Ok(Json(json!(user)))
}
