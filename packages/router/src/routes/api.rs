use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
use diesel::prelude::*;

use axum::Json;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use wabper_common::{util::gen_id, Error};
use wabper_db::models::NewPaste;
use wabper_db::schema::pastes;
use wabper_db::structures::Paste;
pub async fn create_paste(
    Json(pasteinfo): Json<NewPaste>,
    Extension(db): Extension<Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl IntoResponse, Error> {
    let connection: PooledConnection<ConnectionManager<PgConnection>> = db.get()?;
    let NewPaste {
        author,
        body,
        title,
    } = pasteinfo;
    let paste = Paste {
        author,
        title,
        body,
        id: gen_id(),
    };
    diesel::insert_into(pastes::table)
        .values(&paste)
        .get_result::<Paste>(&connection)?;

    Ok((StatusCode::OK, Json(paste)))
}
