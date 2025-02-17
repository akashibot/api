use crate::app::AppState;
use axum::routing::{get, post};
use axum::Router;

pub mod create;
pub mod metadata;
pub mod user;

// todo: add update and delete (tags)

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:guild_id", get(metadata::metadata))
        .route("/", post(create::create))
        .route("/user/list", get(user::metadata))
}
