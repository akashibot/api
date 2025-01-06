use crate::app::AppState;
use axum::routing::{get, post};
use axum::Router;

pub mod metadata;
pub mod create;

// todo: add update and delete (users)

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:id", get(metadata::metadata))
        .route("/", post(create::create))
}