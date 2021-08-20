mod routes;
use actix_web::{web, App, HttpServer};
use routes::{api, fof};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug) {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let manager = r2d2_postgres::PostgresConnectionManager::new(
        "host=db dbname=wabper password=password user=postgres"
            .parse()
            .unwrap(),
        r2d2_postgres::postgres::NoTls,
    );
    let pool = r2d2::Pool::builder().max_size(5).build(manager).unwrap();
    let _ = pool.get().unwrap().batch_execute(
        "CREATE TABLE IF NOT EXISTS Pastes (
            id SERIAL NOT NULL,
            body TEXT NOT NULL,
            author VARCHAR NOT NULL,
            title VARCHAR(80) NOT NULL
        );",
    );
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(api::api())
            .default_service(web::get().to(fof::fof))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
