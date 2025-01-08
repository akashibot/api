use crate::util::errors::forbidden;
use axum::extract::Request;
use axum::response::IntoResponse;
use std::env;

// todo: turn this simple middleware into a more complex one implementing discord oauth2

/// A simple middleware that checks for a valid Bearer token in the Authorization header.
///
/// If the token is valid, the request is passed to the next middleware.
pub async fn simple_auth(req: Request, next: axum::middleware::Next) -> impl IntoResponse {
    let auth_header = req.headers().get("Authorization");
    let valid_bearer = env::var("BEARER_TOKEN").expect("BEARER_TOKEN must be set");

    match auth_header {
        Some(header) => {
            let header_value = header.to_str().unwrap();
            if header_value == format!("Bearer {valid_bearer}") {
                next.run(req).await
            } else {
                forbidden("invalid authorization").into_response()
            }
        }
        None => forbidden("missing authorization").into_response(),
    }
}
