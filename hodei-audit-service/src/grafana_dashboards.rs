//! Grafana Dashboards module
//!
//! This module provides functionality to create and manage Grafana dashboards
//! for monitoring the Hodei Audit Service:
//! - Overview dashboard
//! - Per-tenant metrics dashboard
//! - Performance dashboard
//! - Error tracking dashboard
//! - SLO dashboards (latency, availability)
//! - Alert configurations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub uid: String,
    pub title: String,
    pub tags: Vec<String>,
    pub timezone: String,
    pub refresh: String,
    pub time: TimeRange,
    pub panels: Vec<Panel>,
}

/// Time range for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub from: String,
    pub to: String,
}

/// Panel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub id: Option<u32>,
    pub title: String,
    #[serde(rename = "type")]
    pub panel_type: String,
    pub targets: Vec<Target>,
    pub grid_pos: GridPos,
    pub visualization: VisualizationConfig,
}

/// Query target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub expr: String,
    pub ref_id: String,
    pub legend_format: Option<String>,
}

/// Grid position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridPos {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Visualization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub type_name: String,
    pub field_min_width: Option<u32>,
    pub field_max_width: Option<u32>,
    pub color_mode: Option<String>,
    pub color_scheme: Option<String>,
}

/// Alert rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    pub for_duration: String,
    pub conditions: Vec<Condition>,
    pub annotations: HashMap<String, String>,
    pub labels: HashMap<String, String>,
}

/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub data: Vec<ConditionData>,
    pub operator: String,
}

/// Condition data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionData {
    pub ref_id: String,
    pub query: String,
}

/// GrafanaDashboardManager manages dashboard configurations
#[derive(Debug, Clone)]
pub struct GrafanaDashboardManager {
    dashboards: HashMap<String, DashboardConfig>,
    alert_rules: Vec<AlertRule>,
}

impl GrafanaDashboardManager {
    /// Create a new GrafanaDashboardManager
    pub fn new() -> Self {
        let mut manager = Self {
            dashboards: HashMap::new(),
            alert_rules: Vec::new(),
        };

        // Create default dashboards
        manager.create_overview_dashboard();
        manager.create_per_tenant_dashboard();
        manager.create_performance_dashboard();
        manager.create_error_tracking_dashboard();
        manager.create_slo_dashboards();
        manager.create_alert_rules();

        manager
    }

    /// Create overview dashboard
    fn create_overview_dashboard(&mut self) {
        let dashboard = DashboardConfig {
            uid: "hodei-audit-overview".to_string(),
            title: "Hodei Audit - Overview".to_string(),
            tags: vec!["hodei".to_string(), "audit".to_string(), "overview".to_string()],
            timezone: "browser".to_string(),
            refresh: "30s".to_string(),
            time: TimeRange {
                from: "now-24h".to_string(),
                to: "now".to_string(),
            },
            panels: vec![
                Panel {
                    id: Some(1),
                    title: "Events Received (Last 24h)".to_string(),
                    panel_type: "stat".to_string(),
                    targets: vec![Target {
                        expr: "hodei_audit_events_received_total".to_string(),
                        ref_id: "A".to_string(),
                        legend_format: Some("{{event_type}} - {{status}}".to_string()),
                    }],
                    grid_pos: GridPos { x: 0, y: 0, w: 12, h: 8 },
                    visualization: VisualizationConfig {
                        type_name: "stat".to_string(),
                        field_min_width: None,
                        field_max_width: None,
                        color_mode: Some("value".to_string()),
                        color_scheme: Some("palette-classic".to_string()),
                    },
                },
                Panel {
                    id: Some(2),
                    title: "Processing Latency (p50, p95, p99)".to_string(),
                    panel_type: "graph".to_string(),
                    targets: vec![Target {
                        expr: "histogram_quantile(0.50, hodei_audit_processing_latency_seconds_bucket)".to_string(),
                        ref_id: "A".to_string(),
                        legend_format: Some("p50".to_string()),
                    }],
                    grid_pos: GridPos { x: 12, y: 0, w: 12, h: 8 },
                    visualization: VisualizationConfig {
                        type_name: "graph".to_string(),
                        field_min_width: None,
                        field_max_width: None,
                        color_mode: Some("background".to_string()),
                        color_scheme: Some("palette-classic".to_string()),
                    },
                },
            ],
        };

        self.dashboards.insert("overview".to_string(), dashboard);
    }

    /// Create per-tenant dashboard
    fn create_per_tenant_dashboard(&mut self) {
        let dashboard = DashboardConfig {
            uid: "hodei-audit-tenant".to_string(),
            title: "Hodei Audit - Per-Tenant Metrics".to_string(),
            tags: vec![
                "hodei".to_string(),
                "audit".to_string(),
                "tenant".to_string(),
            ],
            timezone: "browser".to_string(),
            refresh: "30s".to_string(),
            time: TimeRange {
                from: "now-1h".to_string(),
                to: "now".to_string(),
            },
            panels: vec![Panel {
                id: Some(1),
                title: "Events by Tenant".to_string(),
                panel_type: "table".to_string(),
                targets: vec![Target {
                    expr: "hodei_audit_events_received_total".to_string(),
                    ref_id: "A".to_string(),
                    legend_format: Some("{{tenant_id}}".to_string()),
                }],
                grid_pos: GridPos {
                    x: 0,
                    y: 0,
                    w: 24,
                    h: 8,
                },
                visualization: VisualizationConfig {
                    type_name: "table".to_string(),
                    field_min_width: None,
                    field_max_width: None,
                    color_mode: None,
                    color_scheme: None,
                },
            }],
        };

        self.dashboards.insert("per-tenant".to_string(), dashboard);
    }

    /// Create performance dashboard
    fn create_performance_dashboard(&mut self) {
        let dashboard = DashboardConfig {
            uid: "hodei-audit-performance".to_string(),
            title: "Hodei Audit - Performance".to_string(),
            tags: vec![
                "hodei".to_string(),
                "audit".to_string(),
                "performance".to_string(),
            ],
            timezone: "browser".to_string(),
            refresh: "15s".to_string(),
            time: TimeRange {
                from: "now-15m".to_string(),
                to: "now".to_string(),
            },
            panels: vec![
                Panel {
                    id: Some(1),
                    title: "Throughput (events/sec)".to_string(),
                    panel_type: "graph".to_string(),
                    targets: vec![Target {
                        expr: "rate(hodei_audit_events_received_total[1m])".to_string(),
                        ref_id: "A".to_string(),
                        legend_format: Some("{{event_type}}".to_string()),
                    }],
                    grid_pos: GridPos {
                        x: 0,
                        y: 0,
                        w: 12,
                        h: 8,
                    },
                    visualization: VisualizationConfig {
                        type_name: "graph".to_string(),
                        field_min_width: None,
                        field_max_width: None,
                        color_mode: Some("background".to_string()),
                        color_scheme: Some("palette-classic".to_string()),
                    },
                },
                Panel {
                    id: Some(2),
                    title: "Active Connections".to_string(),
                    panel_type: "stat".to_string(),
                    targets: vec![Target {
                        expr: "hodei_audit_active_connections".to_string(),
                        ref_id: "A".to_string(),
                        legend_format: None,
                    }],
                    grid_pos: GridPos {
                        x: 12,
                        y: 0,
                        w: 12,
                        h: 8,
                    },
                    visualization: VisualizationConfig {
                        type_name: "stat".to_string(),
                        field_min_width: None,
                        field_max_width: None,
                        color_mode: Some("value".to_string()),
                        color_scheme: Some("palette-classic".to_string()),
                    },
                },
            ],
        };

        self.dashboards.insert("performance".to_string(), dashboard);
    }

    /// Create error tracking dashboard
    fn create_error_tracking_dashboard(&mut self) {
        let dashboard = DashboardConfig {
            uid: "hodei-audit-errors".to_string(),
            title: "Hodei Audit - Error Tracking".to_string(),
            tags: vec!["hodei".to_string(), "audit".to_string(), "errors".to_string()],
            timezone: "browser".to_string(),
            refresh: "30s".to_string(),
            time: TimeRange {
                from: "now-1h".to_string(),
                to: "now".to_string(),
            },
            panels: vec![
                Panel {
                    id: Some(1),
                    title: "Failed Events by Type".to_string(),
                    panel_type: "piechart".to_string(),
                    targets: vec![Target {
                        expr: "hodei_audit_events_failed_total".to_string(),
                        ref_id: "A".to_string(),
                        legend_format: Some("{{event_type}}".to_string()),
                    }],
                    grid_pos: GridPos { x: 0, y: 0, w: 12, h: 8 },
                    visualization: VisualizationConfig {
                        type_name: "piechart".to_string(),
                        field_min_width: None,
                        field_max_width: None,
                        color_mode: None,
                        color_scheme: None,
                    },
                },
                Panel {
                    id: Some(2),
                    title: "Error Rate (%)".to_string(),
                    panel_type: "graph".to_string(),
                    targets: vec![Target {
                        expr: "rate(hodei_audit_events_failed_total[5m]) / rate(hodei_audit_events_received_total[5m]) * 100".to_string(),
                        ref_id: "A".to_string(),
                        legend_format: Some("Error Rate".to_string()),
                    }],
                    grid_pos: GridPos { x: 12, y: 0, w: 12, h: 8 },
                    visualization: VisualizationConfig {
                        type_name: "graph".to_string(),
                        field_min_width: None,
                        field_max_width: None,
                        color_mode: Some("background".to_string()),
                        color_scheme: Some("palette-classic".to_string()),
                    },
                },
            ],
        };

        self.dashboards
            .insert("error-tracking".to_string(), dashboard);
    }

    /// Create SLO dashboards
    fn create_slo_dashboards(&mut self) {
        // Latency SLO dashboard
        let latency_dashboard = DashboardConfig {
            uid: "hodei-audit-slo-latency".to_string(),
            title: "Hodei Audit - SLO Latency".to_string(),
            tags: vec![
                "hodei".to_string(),
                "audit".to_string(),
                "slo".to_string(),
                "latency".to_string(),
            ],
            timezone: "browser".to_string(),
            refresh: "30s".to_string(),
            time: TimeRange {
                from: "now-7d".to_string(),
                to: "now".to_string(),
            },
            panels: vec![Panel {
                id: Some(1),
                title: "P95 Latency (Target: < 100ms)".to_string(),
                panel_type: "graph".to_string(),
                targets: vec![Target {
                    expr: "histogram_quantile(0.95, hodei_audit_processing_latency_seconds_bucket)"
                        .to_string(),
                    ref_id: "A".to_string(),
                    legend_format: Some("P95 Latency".to_string()),
                }],
                grid_pos: GridPos {
                    x: 0,
                    y: 0,
                    w: 24,
                    h: 8,
                },
                visualization: VisualizationConfig {
                    type_name: "graph".to_string(),
                    field_min_width: None,
                    field_max_width: None,
                    color_mode: Some("background".to_string()),
                    color_scheme: Some("palette-classic".to_string()),
                },
            }],
        };

        self.dashboards
            .insert("slo-latency".to_string(), latency_dashboard);

        // Availability SLO dashboard
        let availability_dashboard = DashboardConfig {
            uid: "hodei-audit-slo-availability".to_string(),
            title: "Hodei Audit - SLO Availability".to_string(),
            tags: vec!["hodei".to_string(), "audit".to_string(), "slo".to_string(), "availability".to_string()],
            timezone: "browser".to_string(),
            refresh: "30s".to_string(),
            time: TimeRange {
                from: "now-7d".to_string(),
                to: "now".to_string(),
            },
            panels: vec![
                Panel {
                    id: Some(1),
                    title: "Uptime % (Target: > 99.9%)".to_string(),
                    panel_type: "stat".to_string(),
                    targets: vec![Target {
                        expr: "(1 - (rate(hodei_audit_events_failed_total[7d]) / rate(hodei_audit_events_received_total[7d]))) * 100".to_string(),
                        ref_id: "A".to_string(),
                        legend_format: Some("Uptime".to_string()),
                    }],
                    grid_pos: GridPos { x: 0, y: 0, w: 12, h: 8 },
                    visualization: VisualizationConfig {
                        type_name: "stat".to_string(),
                        field_min_width: None,
                        field_max_width: None,
                        color_mode: Some("value".to_string()),
                        color_scheme: Some("palette-classic".to_string()),
                    },
                },
            ],
        };

        self.dashboards
            .insert("slo-availability".to_string(), availability_dashboard);
    }

    /// Create alert rules
    fn create_alert_rules(&mut self) {
        // High error rate alert
        self.alert_rules.push(AlertRule {
            name: "High Error Rate".to_string(),
            for_duration: "5m".to_string(),
            conditions: vec![Condition {
                data: vec![ConditionData {
                    ref_id: "A".to_string(),
                    query: "rate(hodei_audit_events_failed_total[5m]) / rate(hodei_audit_events_received_total[5m])".to_string(),
                }],
                operator: "gt".to_string(),
            }],
            annotations: {
                let mut map = HashMap::new();
                map.insert("summary".to_string(), "High error rate detected".to_string());
                map.insert("description".to_string(), "Error rate is {{ $value }}%".to_string());
                map
            },
            labels: {
                let mut map = HashMap::new();
                map.insert("severity".to_string(), "warning".to_string());
                map.insert("service".to_string(), "hodei-audit".to_string());
                map
            },
        });

        // High latency alert
        self.alert_rules.push(AlertRule {
            name: "High Latency".to_string(),
            for_duration: "2m".to_string(),
            conditions: vec![Condition {
                data: vec![ConditionData {
                    ref_id: "A".to_string(),
                    query:
                        "histogram_quantile(0.95, hodei_audit_processing_latency_seconds_bucket)"
                            .to_string(),
                }],
                operator: "gt".to_string(),
            }],
            annotations: {
                let mut map = HashMap::new();
                map.insert("summary".to_string(), "High processing latency".to_string());
                map.insert(
                    "description".to_string(),
                    "P95 latency is {{ $value }}s".to_string(),
                );
                map
            },
            labels: {
                let mut map = HashMap::new();
                map.insert("severity".to_string(), "warning".to_string());
                map.insert("service".to_string(), "hodei-audit".to_string());
                map
            },
        });
    }

    /// Get a dashboard by name
    pub fn get_dashboard(&self, name: &str) -> Option<&DashboardConfig> {
        self.dashboards.get(name)
    }

    /// Get all dashboard names
    pub fn get_dashboard_names(&self) -> Vec<String> {
        self.dashboards.keys().cloned().collect()
    }

    /// Get all alert rules
    pub fn get_alert_rules(&self) -> &[AlertRule] {
        &self.alert_rules
    }

    /// Export dashboard as JSON
    pub fn export_dashboard_json(&self, name: &str) -> Option<String> {
        self.get_dashboard(name)
            .map(|dashboard| serde_json::to_string_pretty(dashboard).unwrap_or_default())
    }

    /// Export all dashboards as JSON
    pub fn export_all_dashboards_json(&self) -> HashMap<String, String> {
        self.dashboards
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    serde_json::to_string_pretty(v).unwrap_or_default(),
                )
            })
            .collect()
    }
}

impl Default for GrafanaDashboardManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_manager_creation() {
        let manager = GrafanaDashboardManager::new();

        let dashboard_names = manager.get_dashboard_names();
        assert!(!dashboard_names.is_empty());
        assert!(dashboard_names.contains(&"overview".to_string()));
    }

    #[test]
    fn test_get_overview_dashboard() {
        let manager = GrafanaDashboardManager::new();

        let dashboard = manager.get_dashboard("overview");
        assert!(dashboard.is_some());

        let dashboard = dashboard.unwrap();
        assert_eq!(dashboard.title, "Hodei Audit - Overview");
        assert_eq!(dashboard.uid, "hodei-audit-overview");
    }

    #[test]
    fn test_get_per_tenant_dashboard() {
        let manager = GrafanaDashboardManager::new();

        let dashboard = manager.get_dashboard("per-tenant");
        assert!(dashboard.is_some());

        let dashboard = dashboard.unwrap();
        assert_eq!(dashboard.title, "Hodei Audit - Per-Tenant Metrics");
        assert!(!dashboard.panels.is_empty());
    }

    #[test]
    fn test_get_performance_dashboard() {
        let manager = GrafanaDashboardManager::new();

        let dashboard = manager.get_dashboard("performance");
        assert!(dashboard.is_some());

        let dashboard = dashboard.unwrap();
        assert_eq!(dashboard.title, "Hodei Audit - Performance");
    }

    #[test]
    fn test_get_error_tracking_dashboard() {
        let manager = GrafanaDashboardManager::new();

        let dashboard = manager.get_dashboard("error-tracking");
        assert!(dashboard.is_some());

        let dashboard = dashboard.unwrap();
        assert_eq!(dashboard.title, "Hodei Audit - Error Tracking");
    }

    #[test]
    fn test_get_slo_dashboards() {
        let manager = GrafanaDashboardManager::new();

        let latency_dashboard = manager.get_dashboard("slo-latency");
        assert!(latency_dashboard.is_some());

        let availability_dashboard = manager.get_dashboard("slo-availability");
        assert!(availability_dashboard.is_some());
    }

    #[test]
    fn test_get_alert_rules() {
        let manager = GrafanaDashboardManager::new();

        let alert_rules = manager.get_alert_rules();
        assert!(!alert_rules.is_empty());

        // Check for high error rate alert
        let has_error_alert = alert_rules
            .iter()
            .any(|rule| rule.name == "High Error Rate");
        assert!(has_error_alert);

        // Check for high latency alert
        let has_latency_alert = alert_rules.iter().any(|rule| rule.name == "High Latency");
        assert!(has_latency_alert);
    }

    #[test]
    fn test_export_dashboard_json() {
        let manager = GrafanaDashboardManager::new();

        let json = manager.export_dashboard_json("overview");
        assert!(json.is_some());

        let json_str = json.unwrap();
        assert!(json_str.contains("Hodei Audit - Overview"));
    }

    #[test]
    fn test_export_all_dashboards_json() {
        let manager = GrafanaDashboardManager::new();

        let all_jsons = manager.export_all_dashboards_json();
        assert!(!all_jsons.is_empty());
        assert!(all_jsons.contains_key("overview"));
        assert!(all_jsons.contains_key("performance"));
        assert!(all_jsons.contains_key("error-tracking"));
    }
}
