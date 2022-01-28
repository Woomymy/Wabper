use axum::body::Body;
use tower_http::trace::{OnRequest, OnResponse};
#[derive(Clone)]
/// Wabper logger for tower_http traits (trasmits all logs to "tracing" macros of the `tracing` crate)
pub struct Logger;

impl OnRequest<Body> for Logger {
    fn on_request(&mut self, request: &axum::http::Request<Body>, _: &tracing::Span) {
        info!("{} {}", request.method(), request.uri())
    }
}

impl<B> OnResponse<B> for Logger {
    fn on_response(
        self,
        response: &axum::http::Response<B>,
        latency: std::time::Duration,
        _: &tracing::Span,
    ) {
        info!(
            "Sent response with status {} in {}ms",
            response.status(),
            latency.as_micros()
        );
    }
}
