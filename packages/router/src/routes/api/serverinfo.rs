use axum::{response::IntoResponse, Json};
use wabper_db::structures::ServerInfo;

/// Get server informations
pub async fn serverinfo() -> impl IntoResponse {
    Json(ServerInfo::default())
}
