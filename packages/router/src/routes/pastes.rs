use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use wabper_common::Error;
use wabper_db::{
    schema::pastes::dsl::{id as pasteid, pastes},
    structures::Paste,
};

pub async fn get_paste(
    Path(id): Path<String>,
    Extension(db): Extension<Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl IntoResponse, Error> {
    let connection: PooledConnection<ConnectionManager<PgConnection>> = db.get()?;
    let paste = pastes.filter(pasteid.eq(id)).first::<Paste>(&connection)?;

    Ok(Json(paste))
}
