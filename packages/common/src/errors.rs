use std::num::ParseIntError;

use axum::body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub struct Error(String, StatusCode);

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self(format!("{}", e), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<axum::Error> for Error {
    fn from(e: axum::Error) -> Self {
        Self(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(e: diesel::ConnectionError) -> Self {
        Self(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        if e == diesel::result::Error::NotFound {
            return Self(e.to_string(), StatusCode::NOT_FOUND);
        }
        Self(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Self(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.1)
            .body(body::boxed(body::Full::from(format!(
                r#"{{ "error": true, "message": "{}" }}"#,
                self.0.replace('"', "\\\"") // Escape quotes in JSON
            ))))
            .unwrap()
    }
}
