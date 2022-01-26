use axum::Server;
use std::net::SocketAddr;
use wabper_common::Error;
use wabper_router::get_axum_router;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
        .serve(get_axum_router()?.into_make_service())
        .await?;

    Ok(())
}
