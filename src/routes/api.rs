use actix_web::{web, HttpRequest, HttpResponse, Responder};
use qstring::QString;
use serde::{Deserialize, Serialize};
/// GET /api/ping: Check if the server is up
async fn ping() -> impl Responder {
    "ping"
}
async fn version() -> impl Responder {
    env!("CARGO_PKG_VERSION")
}
#[derive(Deserialize, Serialize)]
struct CreateQuery {
    body: String,
    author: Option<String>,
    title: String,
}

async fn create(
    req: HttpRequest,
    data: web::Data<
        r2d2::Pool<r2d2_postgres::PostgresConnectionManager<r2d2_postgres::postgres::NoTls>>,
    >,
    info: web::Json<CreateQuery>,
) -> HttpResponse {
    let query_str = QString::from(req.query_string());
    let mut pool = data.get().unwrap();
    let author = info.author.clone().unwrap_or_else(|| String::from("Guest"));

    let paste = pool
        .query(
            "INSERT INTO Pastes (body, author, title) VALUES (
       $1, $2, $3
    ) RETURNING id",
            &[&info.body, &author, &info.title],
        )
        .unwrap();
    let id: i32 = paste[0].get(0);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{{ id: {} }}", id))
}
pub fn api() -> actix_web::Scope {
    web::scope("/api")
        .route("/ping", web::get().to(ping))
        .route("/version", web::get().to(version))
        .route("/create", web::post().to(create))
}
