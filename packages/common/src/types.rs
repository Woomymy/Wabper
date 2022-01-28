use axum::extract::Extension;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

/// Database pool type (PgSQL)
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
/// Axum extension with DbPool inside
pub type DbPoolExtension = Extension<DbPool>;
/// "Pooled" database connection (PgSQL)
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
