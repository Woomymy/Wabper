#[macro_use]
extern crate tracing;
use axum::Server;
use std::net::SocketAddr;
use wabper_common::Error;
use wabper_router::get_axum_router;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Listening on {addr}");

    Server::bind(&addr)
        .serve(get_axum_router()?.into_make_service())
        .await?;

    Ok(())
}
