use actix_web::HttpResponse;
/*
 * Handle 404 (four o four) errors
 * */
pub async fn fof() -> HttpResponse {
    HttpResponse::NotFound().body("Not found!")
}
