use axum::extract::Extension;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

/// Connection manager for Postgres connection
pub type PgConnectionManager = ConnectionManager<PgConnection>;

/// Database pool type (PgSQL)
pub type DbPool = Pool<PgConnectionManager>;

/// Axum extension with DbPool inside
pub type DbPoolExtension = Extension<DbPool>;

/// "Pooled" database connection (PgSQL)
pub type DbConnection = PooledConnection<PgConnectionManager>;
