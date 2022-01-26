//! Wabper router
mod logger;
use logger::Logger;
mod routes;
use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};
#[macro_use]
extern crate tracing;
use routes::ping::ping;
use tower_http::trace::TraceLayer;
use wabper_common::Error;
use wabper_db::db_get_connection;

pub fn get_axum_router() -> Result<Router, Error> {
    let db_client = db_get_connection()?;

    Ok(Router::new()
        .route("/ping", get(ping))
        .route("/api/create", post(routes::api::create_paste))
        .route("/pastes/:id", get(routes::pastes::get_paste))
        .layer(AddExtensionLayer::new(db_client))
        .layer(TraceLayer::new_for_http().on_request(Logger)))
}
