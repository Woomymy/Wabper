//! Wabper router
mod routes;
use axum::{routing::get, Router};

use routes::ping::ping;

pub fn get_axum_router() -> Router {
    Router::new().route("/ping", get(ping))
}
