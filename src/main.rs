mod routes;
use routes::fof;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, get};

async fn hello(req: HttpRequest) -> impl Responder {
    format!("Hello, World!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug) {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route("/", web::get().to(hello))
            .default_service(web::get().to(fof::fof))
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}
