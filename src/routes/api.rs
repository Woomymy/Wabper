use actix_web::{Responder, web};
async fn hello() -> impl Responder {
    "hello"
}
pub fn api() -> actix_web::Scope {
    web::scope("/api")
        .route("/hello", web::get().to(hello))
}
