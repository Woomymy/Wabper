use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;
use wabper_common::{
    types::{DbConnection, DbPoolExtension},
    Error,
};
use wabper_db::{
    schema::pastes::dsl::{id as pasteid, pastes},
    structures::Paste,
};

pub async fn get_paste(
    Path(id): Path<String>,
    Extension(db): DbPoolExtension,
) -> Result<impl IntoResponse, Error> {
    let connection: DbConnection = db.get()?;
    let paste = pastes.filter(pasteid.eq(id)).first::<Paste>(&connection)?;

    Ok(Json(paste.without_delete_pw()))
}
