/// ! Common database and structures utils for wabper

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate tracing;

pub mod models;
pub mod schema;
pub mod structures;

use diesel::r2d2::{ConnectionManager, Pool};
use wabper_common::{
    types::{DbPool, PgConnectionManager},
    util::DB_CONNECTION_TIMEOUT,
    Error,
};

// Embed all migrations
embed_migrations!("migrations");

pub fn get_db_connection() -> Result<DbPool, Error> {
    let db_url = std::env::var("DATABASE_URL")?;
    info!("Database URL: {db_url}");
    let manager: PgConnectionManager = ConnectionManager::new(&db_url);

    let pool = Pool::builder()
        .connection_timeout(DB_CONNECTION_TIMEOUT)
        .build(manager)?;

    info!("Running diesel migrations...");
    if let Err(e) = embedded_migrations::run_with_output(
        &pool.get_timeout(DB_CONNECTION_TIMEOUT)?,
        &mut std::io::stdout(),
    ) {
        error!("Failed to run migrations!");
        error!("{e}");
        panic!("Failed to run DB migrations!");
    }
    info!("Successfully ran migrations!");
    Ok(pool)
}
