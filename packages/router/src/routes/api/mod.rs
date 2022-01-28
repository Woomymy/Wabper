use axum::{routing::get, Router};

mod pastes;
mod ping;
mod serverinfo;

pub fn get_api_router() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .route("/serverinfo", get(serverinfo::serverinfo))
        .nest("/pastes", pastes::get_pastes_router())
}
