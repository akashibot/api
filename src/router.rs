use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use http::{Method, StatusCode};

use crate::app::AppState;
use crate::controllers::*;
use crate::util::errors::not_found;

pub fn build_axum_router(state: AppState) -> Router<()> {
    let router = Router::new()
        .route("/", get(site_metadata::metadata))
        .route("/guilds", post(guilds::create::create))
        .route("/guilds/:id", get(guilds::metadata::metadata));

    router
        .fallback(|method: Method| async move {
            match method {
                Method::HEAD => StatusCode::NOT_FOUND.into_response(),
                _ => not_found().into_response(),
            }
        })
        .with_state(state)
}
