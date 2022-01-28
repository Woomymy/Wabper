//! /api/pastes routes
use axum::{
    extract::{Extension, Path},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;
use wabper_common::{
    types::{DbConnection, DbPoolExtension},
    util::{check_password, gen_deletion_pw, gen_id, hash_string},
    Error,
};
use wabper_db::{
    models::NewPaste,
    schema::pastes::dsl::{id as pasteid, pastes},
    structures::Paste,
};

/**
 * POST /api/pastes/create
 * Creates a new paste and return it has JSON
 */
pub async fn post_paste(
    Json(pasteinfo): Json<NewPaste>,
    Extension(db): DbPoolExtension,
) -> Result<impl IntoResponse, Error> {
    let connection: DbConnection = db.get()?;
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
    diesel::insert_into(pastes)
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

/**
 * GET /api/pastes/:id
 * Fetch paste by <id> and return as JSON
 */
pub async fn get_paste(
    Path(id): Path<String>,
    Extension(db): DbPoolExtension,
) -> Result<impl IntoResponse, Error> {
    let connection: DbConnection = db.get()?;
    let paste = pastes.filter(pasteid.eq(id)).first::<Paste>(&connection)?;

    Ok(Json(paste.without_delete_pw()))
}

/**
 * DELETE /api/pastes/:id
 * Deletes a paste by ID with password
 * Headers:
 * "Authorization: deletionpw"
 */
pub async fn delete_paste(
    Path(id): Path<String>,
    headers: HeaderMap,
    Extension(db): DbPoolExtension,
) -> Result<impl IntoResponse, Error> {
    if !headers.contains_key(header::AUTHORIZATION) {
        return Err(Error::from((
            "No Authorization header!".to_string(),
            StatusCode::FORBIDDEN,
        )));
    }
    if let Some(pw) = headers.get(header::AUTHORIZATION) {
        let pass = pw.to_str()?.to_string();
        let connection: DbConnection = db.get()?;
        let paste = pastes.filter(pasteid.eq(&id)).first::<Paste>(&connection)?;

        if check_password(pass, paste.deletionpw.clone())? {
            diesel::delete(pastes.filter(pasteid.eq(&id))).execute(&connection)?;
            return Ok(Json(paste.without_delete_pw()));
        } else {
            return Err(Error::from((
                "Invalid password!".to_string(),
                StatusCode::FORBIDDEN,
            )));
        }
    }

    Err(Error::from((
        "Unknown error!".to_string(),
        StatusCode::INTERNAL_SERVER_ERROR,
    )))
}
