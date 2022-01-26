use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
use diesel::prelude::*;

use axum::Json;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use wabper_common::Error;
use wabper_db::models::NewPaste;
use wabper_db::schema::pastes;
use wabper_db::schema::pastes::dsl::*;
use wabper_db::structures::Paste;
pub async fn create_paste(
    Json(pasteinfo): Json<NewPaste>,
    Extension(db): Extension<Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl IntoResponse, Error> {
    let connection: PooledConnection<ConnectionManager<PgConnection>> = db.get()?;
    diesel::insert_into(pastes::table)
        .values(&pasteinfo)
        .get_result::<Paste>(&connection)?;
    Ok((StatusCode::OK, Json(pasteinfo)))
}
