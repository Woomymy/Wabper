//! Common wabper lib

use axum::body::{self};
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
#[derive(Debug)]
pub struct Error(String);

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self(format!("{}", e))
    }
}

impl From<axum::Error> for Error {
    fn from(e: axum::Error) -> Self {
        Self(e.to_string())
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self(e.to_string())
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(e: diesel::ConnectionError) -> Self {
        Self(e.to_string())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Self(e.to_string())
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self(e.to_string())
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Self(e.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body::boxed(body::Full::from(format!(
                r#"{{ "error": true, "message": "Internal server error: {}" }}"#,
                self.0.replace('"', "\\\"") // Escape quotes in JSON
            ))))
            .unwrap()
    }
}
