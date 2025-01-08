use crate::app::AppState;
use axum::routing::{delete, get, patch, post};
use axum::Router;

pub mod create;
pub mod delete;
pub mod metadata;
pub mod update;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create::create))
        .route("/:id", get(metadata::metadata))
        .route("/:id", patch(update::update))
        .route("/:id", delete(delete::delete))
}
