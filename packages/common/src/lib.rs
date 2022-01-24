//! Common wabper lib

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
