use crate::app::AppState;
use crate::controllers::*;
use crate::util::errors::not_found;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use http::{Method, StatusCode};

pub fn build_axum_router(state: AppState) -> Router<()> {
    let router = Router::new()
        .route("/", get(site_metadata::metadata))
        .nest("/guilds", guilds::router())
        .nest("/users", users::router())
        .nest("/tags", tags::router());

    router
        .fallback(|method: Method| async move {
            match method {
                Method::HEAD => StatusCode::NOT_FOUND.into_response(),
                _ => not_found().into_response(),
            }
        })
        .with_state(state)
}
