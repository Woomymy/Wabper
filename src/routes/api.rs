use actix_web::{web, Responder};
/// GET /api/ping: Check if the server is up
async fn ping() -> impl Responder {
    "ping"
}
async fn version() -> impl Responder {
    env!("CARGO_PKG_VERSION")
}
pub fn api() -> actix_web::Scope {
    web::scope("/api")
        .route("/ping", web::get().to(ping))
        .route("/version", web::get().to(version))
}
