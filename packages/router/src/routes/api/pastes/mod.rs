use axum::{
    routing::{get, post as postroute},
    Router,
};

pub mod delete;
pub mod get;
pub mod post;
pub fn get_pastes_router() -> Router {
    Router::new()
        .route("/:id", get(get::get_paste).delete(delete::delete_paste))
        .route("/create", postroute(post::create_paste))
}
