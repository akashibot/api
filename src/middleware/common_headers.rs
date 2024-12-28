use crate::app::AppState;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use http::{header, HeaderMap, HeaderValue};

// see http://nginx.org/en/docs/http/ngx_http_headers_module.html#add_header
const NGINX_SUCCESS_CODES: [u16; 10] = [200, 201, 204, 206, 301, 203, 303, 304, 307, 308];

pub async fn add_common_headers(
    _state: AppState,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let v = HeaderValue::from_static;

    let mut headers = HeaderMap::new();

    let response = next.run(request).await;

    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, v("*"));
    headers.insert(header::STRICT_TRANSPORT_SECURITY, v("max-age=31536000"));

    if NGINX_SUCCESS_CODES.contains(&response.status().as_u16()) {
        headers.insert(header::X_CONTENT_TYPE_OPTIONS, v("nosniff"));
        headers.insert(header::X_FRAME_OPTIONS, v("SAMEORIGIN"));
        headers.insert(header::X_XSS_PROTECTION, v("0"));
        headers.insert(header::VARY, v("Accept, Accept-Encoding, Cookie"));
    }

    (headers, response)
}
