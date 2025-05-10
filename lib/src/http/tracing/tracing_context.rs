use tracing::{Level, Span, span};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TraceContext {
    pub trace_id: String,
    pub request_id: String,
}

impl TraceContext {
    /// Create TraceContext from optional upstream `trace_id`
    pub fn new(trace_id: Option<&str>) -> Self {
        let trace_id = trace_id
            .map(str::to_owned)
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let request_id = Uuid::new_v4().to_string();

        TraceContext {
            trace_id,
            request_id,
        }
    }

    /// Create and enter a tracing span with MDC context
    pub fn span(&self) -> Span {
        span!(
            Level::INFO,
            "http_request",
            trace_id = %self.trace_id,
            request_id = %self.request_id,
        )
    }
}
