pub mod app;
mod common_headers;
mod debug;
pub mod log_request;
pub mod normalize_path;
pub mod real_ip;
pub mod simple_auth;

// use ::sentry::integrations::tower as sentry_tower;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::Router;
use axum_extra::either::Either;
use axum_extra::middleware::option_layer;
use std::time::Duration;
use tower::layer::util::Identity;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::{CompressionLayer, CompressionLevel};
use tower_http::timeout::{RequestBodyTimeoutLayer, TimeoutLayer};

use crate::app::AppState;
use crate::Env;

pub fn apply_axum_middleware(state: AppState, router: Router<()>) -> Router {
    let config = &state.0.config;
    let env = config.env();

    let middleware_1 = tower::ServiceBuilder::new()
        .layer(from_fn(real_ip::middleware))
        .layer(CatchPanicLayer::new())
        .layer(from_fn(log_request::log_requests))
        .layer(conditional_layer(env == Env::Development, || {
            from_fn(debug::debug_requests)
        }));

    let middleware_2 = tower::ServiceBuilder::new()
        .layer(from_fn(simple_auth::simple_auth))
        .layer(from_fn_with_state(
            state.clone(),
            common_headers::add_common_headers,
        ))
        .layer(AddExtensionLayer::new(state.clone()));

    router
        .layer(middleware_2)
        .layer(middleware_1)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new().quality(CompressionLevel::Fastest))
}

pub fn conditional_layer<L, F: FnOnce() -> L>(condition: bool, layer: F) -> Either<L, Identity> {
    option_layer(condition.then(layer))
}
