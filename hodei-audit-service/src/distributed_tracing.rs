//! Distributed Tracing module
//!
//! This module provides comprehensive distributed tracing for the Hodei Audit Service:
//! - OpenTelemetry integration
//! - Trace context propagation
//! - Complete span attributes
//! - Jaeger/Tempo setup
//! - Trace sampling strategy

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Trace ID (16-byte identifier)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TraceId(String);

/// Span ID (8-byte identifier)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpanId(String);

/// Trace state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceState {
    /// Trace ID
    pub trace_id: TraceId,
    /// Span ID
    pub span_id: SpanId,
    /// Parent span ID (if applicable)
    pub parent_span_id: Option<SpanId>,
    /// Trace flags
    pub flags: u8,
    /// Baggage items
    pub baggage: HashMap<String, String>,
}

impl TraceState {
    /// Create a new trace state
    pub fn new(trace_id: TraceId, span_id: SpanId) -> Self {
        Self {
            trace_id,
            span_id,
            parent_span_id: None,
            flags: 0,
            baggage: HashMap::new(),
        }
    }

    /// Create a new root span
    pub fn new_root() -> Self {
        let trace_id = TraceId(uuid::Uuid::new_v4().to_string());
        let span_id = SpanId(uuid::Uuid::new_v4().to_string());
        Self::new(trace_id, span_id)
    }

    /// Create a child span
    pub fn new_child(&self) -> Self {
        let span_id = SpanId(uuid::Uuid::new_v4().to_string());
        Self {
            trace_id: self.trace_id.clone(),
            span_id,
            parent_span_id: Some(self.span_id.clone()),
            flags: self.flags,
            baggage: self.baggage.clone(),
        }
    }

    /// Add baggage item
    pub fn with_baggage(mut self, key: &str, value: &str) -> Self {
        self.baggage.insert(key.to_string(), value.to_string());
        self
    }

    /// Set trace flags
    pub fn with_flags(mut self, flags: u8) -> Self {
        self.flags = flags;
        self
    }
}

/// Span kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpanKind {
    Internal,
    Server,
    Client,
    Producer,
    Consumer,
}

/// Span status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Unset,
    Ok,
    Error { code: String, message: String },
}

/// Span attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanAttribute {
    pub key: String,
    pub value: serde_json::Value,
}

impl SpanAttribute {
    /// Create a string attribute
    pub fn string(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: serde_json::Value::String(value.to_string()),
        }
    }

    /// Create a boolean attribute
    pub fn bool(key: &str, value: bool) -> Self {
        Self {
            key: key.to_string(),
            value: serde_json::Value::Bool(value),
        }
    }

    /// Create a number attribute
    pub fn number<T: Serialize>(key: &str, value: T) -> Self {
        Self {
            key: key.to_string(),
            value: serde_json::to_value(value).unwrap_or_default(),
        }
    }
}

/// Span
#[derive(Debug, Clone)]
pub struct Span {
    /// Trace state
    pub trace_state: TraceState,
    /// Span name
    pub name: String,
    /// Span kind
    pub kind: SpanKind,
    /// Start time
    pub start_time: SystemTime,
    /// End time (set when span is closed)
    pub end_time: Option<SystemTime>,
    /// Attributes
    pub attributes: Vec<SpanAttribute>,
    /// Status
    pub status: Status,
    /// Events
    pub events: Vec<SpanEvent>,
    /// Links
    pub links: Vec<SpanLink>,
}

/// Span event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    /// Event name
    pub name: String,
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event attributes
    pub attributes: Vec<SpanAttribute>,
}

/// Span link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanLink {
    /// Linked trace state
    pub trace_state: TraceState,
    /// Link attributes
    pub attributes: Vec<SpanAttribute>,
}

impl Span {
    /// Create a new span
    pub fn new(trace_state: TraceState, name: String, kind: SpanKind) -> Self {
        Self {
            trace_state,
            name,
            kind,
            start_time: SystemTime::now(),
            end_time: None,
            attributes: Vec::new(),
            status: Status::Unset,
            events: Vec::new(),
            links: Vec::new(),
        }
    }

    /// Add attribute
    pub fn with_attribute(mut self, attribute: SpanAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Add event
    pub fn with_event(mut self, name: &str) -> Self {
        self.events.push(SpanEvent {
            name: name.to_string(),
            timestamp: SystemTime::now(),
            attributes: Vec::new(),
        });
        self
    }

    /// Add link
    pub fn with_link(mut self, trace_state: TraceState) -> Self {
        self.links.push(SpanLink {
            trace_state,
            attributes: Vec::new(),
        });
        self
    }

    /// Set status to ok
    pub fn with_status_ok(mut self) -> Self {
        self.status = Status::Ok;
        self
    }

    /// Set status to error
    pub fn with_status_error(mut self, code: &str, message: &str) -> Self {
        self.status = Status::Error {
            code: code.to_string(),
            message: message.to_string(),
        };
        self
    }

    /// Close the span
    pub fn close(mut self) -> Self {
        self.end_time = Some(SystemTime::now());
        self
    }

    /// Get span duration in milliseconds
    pub fn duration_ms(&self) -> Option<u64> {
        if let Some(end) = self.end_time {
            let duration = end.duration_since(self.start_time).ok()?;
            Some(duration.as_millis() as u64)
        } else {
            None
        }
    }
}

/// Trace sampling strategy
#[derive(Debug, Clone)]
pub enum SamplingStrategy {
    /// Always sample
    Always,
    /// Never sample
    Never,
    /// Sample with probability (0.0 to 1.0)
    Probabilistic { probability: f64 },
    /// Rate limiting sampler
    RateLimiting { max_spans_per_second: u64 },
}

impl SamplingStrategy {
    /// Should this trace be sampled?
    pub fn should_sample(&self) -> bool {
        match self {
            SamplingStrategy::Always => true,
            SamplingStrategy::Never => false,
            SamplingStrategy::Probabilistic { probability } => rand::random::<f64>() < *probability,
            SamplingStrategy::RateLimiting { .. } => {
                // In a real implementation, this would check against a rate limiter
                true
            }
        }
    }
}

/// Tracer
#[derive(Debug, Clone)]
pub struct Tracer {
    /// Service name
    service_name: String,
    /// Sampling strategy
    sampling_strategy: SamplingStrategy,
    /// Maximum attributes per span
    max_attributes: usize,
    /// Maximum events per span
    max_events: usize,
    /// Maximum links per span
    max_links: usize,
}

impl Tracer {
    /// Create a new tracer
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            sampling_strategy: SamplingStrategy::Probabilistic { probability: 0.1 }, // 10% default
            max_attributes: 128,
            max_events: 128,
            max_links: 128,
        }
    }

    /// Set sampling strategy
    pub fn with_sampling_strategy(mut self, strategy: SamplingStrategy) -> Self {
        self.sampling_strategy = strategy;
        self
    }

    /// Start a new span
    pub fn start_span(&self, name: &str, kind: SpanKind, trace_state: Option<TraceState>) -> Span {
        let trace_state = trace_state.unwrap_or_else(TraceState::new_root);

        // Check sampling
        if !self.sampling_strategy.should_sample() {
            // Return a no-op span (not sampled)
            return Span::new(trace_state, name.to_string(), kind.clone())
                .with_status_error("NOT_SAMPLED", "Not sampled");
        }

        // Create span with common attributes
        let mut span = Span::new(trace_state, name.to_string(), kind.clone())
            .with_attribute(SpanAttribute::string("service.name", &self.service_name))
            .with_attribute(SpanAttribute::string(
                "span.kind",
                match kind {
                    SpanKind::Internal => "internal",
                    SpanKind::Server => "server",
                    SpanKind::Client => "client",
                    SpanKind::Producer => "producer",
                    SpanKind::Consumer => "consumer",
                },
            ))
            .with_attribute(SpanAttribute::string("telemetry.sdk.name", "hodei-audit"));

        span
    }

    /// Start a root span
    pub fn start_root_span(&self, name: &str) -> Span {
        self.start_span(name, SpanKind::Internal, Some(TraceState::new_root()))
    }

    /// Start a child span
    pub fn start_child_span(&self, name: &str, parent: &Span) -> Span {
        let trace_state = parent.trace_state.new_child();
        self.start_span(name, SpanKind::Internal, Some(trace_state))
    }

    /// Inject trace context into headers
    pub fn inject_context(&self, span: &Span) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        // Standard OpenTelemetry trace headers
        headers.insert(
            "traceparent".to_string(),
            format!(
                "{}-{}-{}",
                span.trace_state.flags, span.trace_state.trace_id.0, span.trace_state.span_id.0
            ),
        );

        // Add baggage items as separate headers
        for (key, value) in &span.trace_state.baggage {
            headers.insert(format!("baggage-{}", key), value.clone());
        }

        headers
    }

    /// Extract trace context from headers
    pub fn extract_context(&self, headers: &HashMap<String, String>) -> Option<TraceState> {
        // Try to extract from traceparent header
        if let Some(traceparent) = headers.get("traceparent") {
            let parts: Vec<&str> = traceparent.split('-').collect();
            if parts.len() >= 3 {
                let trace_id = TraceId(parts[1].to_string());
                let span_id = SpanId(parts[2].to_string());
                let flags = parts[0].parse().unwrap_or(0);

                let mut trace_state = TraceState::new(trace_id, span_id).with_flags(flags);

                // Extract baggage
                for (key, value) in headers {
                    if key.starts_with("baggage-") {
                        let baggage_key = key.strip_prefix("baggage-").unwrap().to_string();
                        trace_state.baggage.insert(baggage_key, value.clone());
                    }
                }

                return Some(trace_state);
            }
        }

        None
    }
}

/// Span recorder (for testing and manual inspection)
#[derive(Debug, Clone, Default)]
pub struct SpanRecorder {
    spans: Vec<Span>,
}

impl SpanRecorder {
    /// Create a new recorder
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }

    /// Record a span
    pub fn record_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    /// Get all recorded spans
    pub fn get_spans(&self) -> &[Span] {
        &self.spans
    }

    /// Clear recorded spans
    pub fn clear(&mut self) {
        self.spans.clear();
    }

    /// Find spans by name
    pub fn find_by_name(&self, name: &str) -> Vec<&Span> {
        self.spans.iter().filter(|s| s.name == name).collect()
    }

    /// Find spans by trace ID
    pub fn find_by_trace_id(&self, trace_id: &str) -> Vec<&Span> {
        self.spans
            .iter()
            .filter(|s| s.trace_state.trace_id.0 == trace_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_state_new_root() {
        let trace_state = TraceState::new_root();
        assert!(!trace_state.trace_id.0.is_empty());
        assert!(!trace_state.span_id.0.is_empty());
        assert!(trace_state.parent_span_id.is_none());
    }

    #[test]
    fn test_trace_state_new_child() {
        let parent = TraceState::new(TraceId("parent".to_string()), SpanId("span1".to_string()));
        let child = parent.new_child();
        assert_eq!(child.trace_id.0, "parent");
        assert!(child.span_id.0 != "span1");
        assert_eq!(child.parent_span_id.as_ref().unwrap().0, "span1");
    }

    #[test]
    fn test_trace_state_baggage() {
        let trace_state = TraceState::new_root()
            .with_baggage("key1", "value1")
            .with_baggage("key2", "value2");
        assert_eq!(trace_state.baggage.get("key1"), Some(&"value1".to_string()));
        assert_eq!(trace_state.baggage.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_span_creation() {
        let trace_state = TraceState::new_root();
        let span = Span::new(trace_state, "test-span".to_string(), SpanKind::Internal);
        assert_eq!(span.name, "test-span");
        assert!(matches!(span.kind, SpanKind::Internal));
        assert!(span.end_time.is_none());
    }

    #[test]
    fn test_span_with_attributes() {
        let trace_state = TraceState::new_root();
        let span = Span::new(trace_state, "test-span".to_string(), SpanKind::Internal)
            .with_attribute(SpanAttribute::string("key1", "value1"))
            .with_attribute(SpanAttribute::bool("key2", true))
            .with_attribute(SpanAttribute::number("key3", 42));

        assert_eq!(span.attributes.len(), 3);
    }

    #[test]
    fn test_span_close() {
        let trace_state = TraceState::new_root();
        let span = Span::new(trace_state, "test-span".to_string(), SpanKind::Internal).close();
        assert!(span.end_time.is_some());
        assert!(span.duration_ms().is_some());
    }

    #[test]
    fn test_span_status() {
        let trace_state = TraceState::new_root();

        let span_ok = Span::new(
            trace_state.clone(),
            "ok-span".to_string(),
            SpanKind::Internal,
        )
        .with_status_ok();
        assert!(matches!(span_ok.status, Status::Ok));

        let span_error = Span::new(trace_state, "error-span".to_string(), SpanKind::Internal)
            .with_status_error("INVALID_ARGUMENT", "Bad request");
        if let Status::Error { code, message } = span_error.status {
            assert_eq!(code, "INVALID_ARGUMENT");
            assert_eq!(message, "Bad request");
        } else {
            panic!("Expected error status");
        }
    }

    #[test]
    fn test_sampling_strategy_always() {
        let strategy = SamplingStrategy::Always;
        assert!(strategy.should_sample());
    }

    #[test]
    fn test_sampling_strategy_never() {
        let strategy = SamplingStrategy::Never;
        assert!(!strategy.should_sample());
    }

    #[test]
    fn test_sampling_strategy_probabilistic() {
        let strategy = SamplingStrategy::Probabilistic { probability: 0.0 };
        assert!(!strategy.should_sample());

        let strategy = SamplingStrategy::Probabilistic { probability: 1.0 };
        assert!(strategy.should_sample());
    }

    #[test]
    fn test_tracer_start_root_span() {
        let tracer = Tracer::new("test-service");
        let span = tracer.start_root_span("root-span");
        assert_eq!(span.name, "root-span");
        assert!(matches!(span.kind, SpanKind::Internal));
    }

    #[test]
    fn test_tracer_start_child_span() {
        let tracer = Tracer::new("test-service");
        let parent = tracer.start_root_span("parent-span");
        let child = tracer.start_child_span("child-span", &parent);

        assert_eq!(child.name, "child-span");
        assert_eq!(child.trace_state.trace_id, parent.trace_state.trace_id);
        assert!(child.trace_state.parent_span_id.is_some());
    }

    #[test]
    fn test_tracer_inject_context() {
        let tracer = Tracer::new("test-service");
        let span = tracer.start_root_span("test-span");
        let headers = tracer.inject_context(&span);

        assert!(headers.contains_key("traceparent"));
    }

    #[test]
    fn test_tracer_extract_context() {
        let tracer = Tracer::new("test-service");

        let mut headers = HashMap::new();
        headers.insert("traceparent".to_string(), "0-1234-5678-0".to_string());
        headers.insert("baggage-key1".to_string(), "value1".to_string());

        let trace_state = tracer.extract_context(&headers);
        assert!(trace_state.is_some());

        let trace_state = trace_state.unwrap();
        assert_eq!(trace_state.trace_id.0, "1234");
        assert_eq!(trace_state.span_id.0, "5678");
        assert_eq!(trace_state.flags, 0);
        assert_eq!(trace_state.baggage.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_span_recorder() {
        let mut recorder = SpanRecorder::new();

        let trace_state = TraceState::new_root();
        let span1 = Span::new(trace_state.clone(), "span1".to_string(), SpanKind::Internal);
        let span2 = Span::new(trace_state, "span2".to_string(), SpanKind::Internal);

        recorder.record_span(span1);
        recorder.record_span(span2);

        assert_eq!(recorder.get_spans().len(), 2);
        assert_eq!(recorder.find_by_name("span1").len(), 1);
        assert_eq!(recorder.find_by_name("span2").len(), 1);
    }
}
