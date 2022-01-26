use axum::{routing::get, Router};

mod pastes;
mod ping;

pub fn get_api_router() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .nest("/pastes", pastes::get_pastes_router())
}
