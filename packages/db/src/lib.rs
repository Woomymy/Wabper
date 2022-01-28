/// ! Common database and structures utils for wabper

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

pub mod models;
pub mod schema;
pub mod structures;

use diesel::r2d2::{ConnectionManager, Pool};
use wabper_common::{types::PgConnectionManager, Error};

pub fn get_db_connection() -> Result<diesel::r2d2::Pool<PgConnectionManager>, Error> {
    let db_url = std::env::var("DATABASE_URL")?;
    info!("Database URL: {db_url}");
    let manager: PgConnectionManager = ConnectionManager::new(&db_url);

    Ok(Pool::new(manager)?)
}
