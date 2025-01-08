use crate::app::AppState;
use axum::routing::{get, post};
use axum::Router;

pub mod create;
pub mod metadata;

// todo: add update and delete (users)

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:id", get(metadata::metadata))
        .route("/", post(create::create))
}
