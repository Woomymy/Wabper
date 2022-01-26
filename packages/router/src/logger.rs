use axum::body::Body;
use tower_http::trace::OnRequest;
#[derive(Clone)]
pub struct Logger;

impl OnRequest<Body> for Logger {
    fn on_request(&mut self, request: &axum::http::Request<Body>, _: &tracing::Span) {
        info!("{} {}", request.method(), request.uri())
    }
}
