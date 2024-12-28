use app::{App, AppState};
use router::build_axum_router;
use std::sync::Arc;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

pub mod app;
pub mod config;
pub mod controllers;
pub mod database;
pub mod headers;
pub mod middleware;
pub mod real_ip;
mod router;
pub mod util;

/// Used for setting different values depending on whether the app is being run in production,
/// in development, or for testing.
///
/// The app's `config.env` value is set in *src/bin/server.rs* to `Production` if the environment
/// variable `COOLIFY_URL` is set and `Development` otherwise. `config.env` is set to `Test`
/// unconditionally in *src/test/all.rs*.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Env {
    Development,
    Test,
    Production,
}

/// Configures routes, sessions, logging, and other middleware.
///
/// Called from *src/bin/server.rs*.
pub fn build_handler(app: Arc<App>) -> axum::Router {
    let state = AppState(app);

    let axum_router = build_axum_router(state.clone());
    middleware::apply_axum_middleware(state, axum_router)
}
