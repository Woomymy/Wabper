use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
use diesel::prelude::*;

use axum::Json;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use wabper_common::{
    util::{gen_deletion_pw, gen_id, hash_string},
    Error,
};
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
    let deletepw = gen_deletion_pw();
    let id = gen_id();
    let paste = Paste {
        author: author.clone(),
        title: title.clone(),
        body: body.clone(),
        id: id.clone(),
        deletionpw: deletepw.clone(),
    };
    diesel::insert_into(pastes::table)
        .values(&Paste {
            author,
            title,
            body,
            id,
            deletionpw: hash_string(deletepw)?,
        })
        .get_result::<Paste>(&connection)?;

    Ok((StatusCode::OK, Json(paste)))
}
