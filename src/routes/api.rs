use actix_web::{web, Responder};
/// GET /api/ping: Check if the server is up
async fn ping() -> impl Responder {
    "ping"
}
pub fn api() -> actix_web::Scope {
    web::scope("/api").route("/ping", web::get().to(ping))
}
