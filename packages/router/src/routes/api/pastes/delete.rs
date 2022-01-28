use axum::{
    extract::{Extension, Path},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use wabper_common::{util::check_password, Error};
use wabper_db::{
    schema::pastes::dsl::{id as pasteid, pastes},
    structures::Paste,
};

pub async fn delete_paste(
    Path(id): Path<String>,
    headers: HeaderMap,
    Extension(db): Extension<Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl IntoResponse, Error> {
    if !headers.contains_key(header::AUTHORIZATION) {
        return Err(Error::from((
            "No Authorization header!".to_string(),
            StatusCode::FORBIDDEN,
        )));
    }
    if let Some(pw) = headers.get(header::AUTHORIZATION) {
        let pass = pw.to_str()?.to_string();
        let connection: PooledConnection<ConnectionManager<PgConnection>> = db.get()?;
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
