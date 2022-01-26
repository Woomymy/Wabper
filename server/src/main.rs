#[macro_use]
extern crate tracing;
use axum::Server;
use std::net::SocketAddr;
use wabper_common::Error;
use wabper_router::get_axum_router;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let listen_port = std::env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse::<u16>()?;

    let addr = SocketAddr::from(([0, 0, 0, 0], listen_port));
    info!("Listening on {addr}");

    Server::bind(&addr)
        .serve(get_axum_router()?.into_make_service())
        .await?;

    Ok(())
}
