//! Vector integration module
//!
//! This module provides the VectorForwarder client for sending audit events
//! to Vector.dev for multi-sink distribution.

pub mod error;
#[cfg(feature = "vector-metrics")]
pub mod metrics;
pub mod sink_manager;
pub mod vector_forwarder;

pub use error::{VectorError, VectorResult};

#[cfg(feature = "vector-metrics")]
pub use metrics::{
    VectorHealthStatus, VectorMetrics, VectorMetricsCollector, VectorMetricsSummary,
};

pub use sink_manager::{VectorSinkConfig, VectorSinkManager, VectorSinkType, create_default_sinks};
pub use vector_forwarder::{VectorForwarder, VectorForwarderConfig};
