use axum::response::IntoResponse;
use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::error::Error;
use std::fmt;

mod json;

use axum::Extension;
use reqwest::StatusCode;
use tracing::error;

use crate::middleware::log_request::ErrorField;

pub(crate) use json::custom;

pub type BoxedAppError = Box<dyn AppError>;

/// Return an error with status 400 and the provided description as JSON
pub fn bad_request<S: ToString>(error: S) -> BoxedAppError {
    custom(StatusCode::BAD_REQUEST, error.to_string())
}

pub fn forbidden(detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    custom(StatusCode::FORBIDDEN, detail)
}

pub fn not_found() -> BoxedAppError {
    custom(StatusCode::NOT_FOUND, "not found")
}

/// Returns an error with status 500 and the provided description as JSON
pub fn server_error<S: ToString>(error: S) -> BoxedAppError {
    custom(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

/// Returns an error with status 503 and the provided description as JSON
pub fn service_unavailable() -> BoxedAppError {
    custom(StatusCode::SERVICE_UNAVAILABLE, "Service unavailable")
}

// =============================================================================
// AppError trait

pub trait AppError: Send + fmt::Display + fmt::Debug + 'static {
    /// Generate an HTTP response for the error
    ///
    /// If none is returned, the error will bubble up the middleware stack
    /// where it is eventually logged and turned into a status 500 response.
    fn response(&self) -> axum::response::Response;

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl dyn AppError {
    pub fn is<T: Any>(&self) -> bool {
        self.get_type_id() == TypeId::of::<T>()
    }
}

impl AppError for BoxedAppError {
    fn response(&self) -> axum::response::Response {
        (**self).response()
    }

    fn get_type_id(&self) -> TypeId {
        (**self).get_type_id()
    }
}

impl IntoResponse for BoxedAppError {
    fn into_response(self) -> axum::response::Response {
        self.response()
    }
}

pub type AppResult<T> = Result<T, BoxedAppError>;

// =============================================================================
// Error impls

impl<E: Error + Send + 'static> AppError for E {
    fn response(&self) -> axum::response::Response {
        error!(error = %self, "Internal Server Error");

        // sentry::capture_error(self);

        server_error_response(self.to_string())
    }
}

impl<E, T> From<oauth2::RequestTokenError<E, T>> for BoxedAppError
where
    T: oauth2::ErrorResponse + 'static,
    E: Error + Send + 'static,
{
    fn from(err: oauth2::RequestTokenError<E, T>) -> BoxedAppError {
        match err {
            oauth2::RequestTokenError::Request(err) => Box::new(err),
            oauth2::RequestTokenError::Parse(err, _) => {
                custom(StatusCode::BAD_REQUEST, err.to_string())
            }
            oauth2::RequestTokenError::Other(err) => {
                custom(StatusCode::BAD_REQUEST, err.to_string())
            }
            oauth2::RequestTokenError::ServerResponse(_) => {
                custom(StatusCode::BAD_REQUEST, "Invalid response from server")
            }
        }
    }
}

impl From<url::ParseError> for BoxedAppError {
    fn from(err: url::ParseError) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<sqlx::Error> for BoxedAppError {
    fn from(err: sqlx::Error) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<serde_json::Error> for BoxedAppError {
    fn from(err: serde_json::Error) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<reqwest::Error> for BoxedAppError {
    fn from(err: reqwest::Error) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<std::io::Error> for BoxedAppError {
    fn from(err: std::io::Error) -> BoxedAppError {
        Box::new(err)
    }
}

// Conversion from anyhow::Error to BoxedAppError
impl From<anyhow::Error> for BoxedAppError {
    fn from(err: anyhow::Error) -> BoxedAppError {
        Box::new(AnyhowWrapper(err))
    }
}

// Wrapper for anyhow::Error
struct AnyhowWrapper(anyhow::Error);

impl fmt::Debug for AnyhowWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for AnyhowWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl AppError for AnyhowWrapper {
    fn response(&self) -> axum::response::Response {
        error!(error = %self.0, "Internal Server Error");
        server_error_response(self.0.to_string())
    }
}

// =============================================================================
// Internal error for use with `chain_error`

#[derive(Debug)]
struct InternalAppError {
    description: String,
}

impl fmt::Display for InternalAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)?;
        Ok(())
    }
}

impl AppError for InternalAppError {
    fn response(&self) -> axum::response::Response {
        error!(error = %self.description, "Internal Server Error");

        // sentry::capture_message(&self.description, sentry::Level::Error);

        server_error_response(self.description.to_string())
    }
}

pub fn internal<S: ToString>(error: S) -> BoxedAppError {
    Box::new(InternalAppError {
        description: error.to_string(),
    })
}

fn server_error_response(error: String) -> axum::response::Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Extension(ErrorField(error)),
        "Internal Server Error",
    )
        .into_response()
}
