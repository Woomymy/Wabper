mod routes;
use routes::{fof, api};
use actix_web::{web, App, HttpServer};

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
            .service(api::api())
            .default_service(web::get().to(fof::fof))
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}
