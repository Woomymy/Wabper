use axum::{
    routing::{get, post},
    Router,
};

mod pastes;
mod ping;
mod serverinfo;

/// Get router for /api/ routes
pub fn get_api_router() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .route("/serverinfo", get(serverinfo::serverinfo))
        .route(
            "/pastes/:id",
            get(pastes::get::get_paste).delete(pastes::delete::delete_paste),
        )
        .route("/pastes/create", post(pastes::post::create_paste))
}
