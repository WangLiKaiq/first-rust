use tracing::{Level, Span, span};
use uuid::Uuid;

/// Creates a new span with request_id and trace_id for MDC-like context
pub fn create_request_span() -> (Span, String, String) {
    let request_id = Uuid::new_v4().to_string();
    let trace_id = Uuid::new_v4().to_string();

    let span = span!(
        Level::INFO,
        "request_span",
        request_id = %request_id,
        trace_id = %trace_id,
    );

    (span, request_id, trace_id)
}
