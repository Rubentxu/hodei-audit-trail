//! HRN (Hodei Resource Name) generation y resolution
//!
//! Este módulo proporciona la funcionalidad para generar HRNs desde request paths
//! y resolver metadata asociada.

use crate::config::{HrnMetadata, HrnResolver};
use crate::error::AuditError;
use http::Method;
use std::collections::HashMap;
use std::sync::Arc;

/// Estructura para representar un HRN
#[derive(Debug, Clone, PartialEq)]
pub struct Hrn {
    /// El string del HRN
    value: String,
    /// Componentes del HRN
    components: HrnComponents,
}

#[derive(Debug, Clone, PartialEq)]
struct HrnComponents {
    service: String,
    tenant: String,
    scope: String,
    resource_type: String,
    resource_id: String,
}

impl Hrn {
    /// Crear un HRN desde string
    pub fn parse(value: &str) -> Result<Self, AuditError> {
        // Formato: hrn:hodei:{service}:{tenant}:{scope}:{resource_type}/{resource_id}
        let parts: Vec<&str> = value.split(':').collect();

        if parts.len() < 6 {
            return Err(AuditError::HrnError(format!(
                "Invalid HRN format: {}. Expected hrn:hodei:service:tenant:scope:resource",
                value
            )));
        }

        if parts[0] != "hrn" || parts[1] != "hodei" {
            return Err(AuditError::HrnError(format!(
                "Invalid HRN prefix: expected 'hrn:hodei', got '{}'",
                parts[0..=1].join(":")
            )));
        }

        // Split resource by '/' to get resource_type and resource_id
        let resource_parts: Vec<&str> = parts[5].split('/').collect();
        let (resource_type, resource_id) = if resource_parts.len() == 2 {
            (resource_parts[0].to_string(), resource_parts[1].to_string())
        } else {
            // If no '/', treat the whole part as resource_type with no specific ID
            (parts[5].to_string(), "*".to_string())
        };

        let components = HrnComponents {
            service: parts[2].to_string(),
            tenant: parts[3].to_string(),
            scope: parts[4].to_string(),
            resource_type,
            resource_id,
        };

        Ok(Self {
            value: value.to_string(),
            components,
        })
    }

    /// Obtener el valor string
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Obtener el tipo de servicio
    pub fn service(&self) -> &str {
        &self.components.service
    }

    /// Obtener el tenant
    pub fn tenant(&self) -> &str {
        &self.components.tenant
    }

    /// Obtener el tipo de recurso
    pub fn resource_type(&self) -> &str {
        &self.components.resource_type
    }

    /// Obtener el ID del recurso
    pub fn resource_id(&self) -> &str {
        &self.components.resource_id
    }
}

impl std::fmt::Display for Hrn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Generar HRN desde método y path
pub fn generate_hrn_from_path(
    method: &Method,
    path: &str,
    tenant_id: Option<&str>,
) -> Result<Hrn, AuditError> {
    let tenant = tenant_id.unwrap_or("unknown");

    // Mapeo de paths a patrones HRN
    let (service_type, resource_type, resource_id) = match path {
        // verified-permissions endpoints
        p if p.starts_with("/v1/policy-stores/") => {
            let policy_store_id =
                extract_id_from_path(p, 2).unwrap_or_else(|| "default".to_string());
            ("verified-permissions", "policy-store", policy_store_id)
        }
        p if p.starts_with("/v1/policy-stores") => {
            ("verified-permissions", "policy-store", "list".to_string())
        }
        p if p.starts_with("/v1/authorize") => {
            ("verified-permissions", "authorization", "check".to_string())
        }

        // API service endpoints
        p if p.starts_with("/api/v1/users/") => {
            let user_id = extract_id_from_path(p, 3).unwrap_or_else(|| "*".to_string());
            ("api", "user", user_id)
        }
        p if p.starts_with("/api/v1/") => {
            let api_path = p.trim_start_matches("/api/v1/");
            let parts: Vec<&str> = api_path.split('/').collect();
            if parts.len() >= 2 {
                let (resource_type, resource_id) = match parts[0] {
                    "users" => ("user", parts[1].to_string()),
                    "products" => ("product", parts[1].to_string()),
                    "orders" => ("order", parts[1].to_string()),
                    _ => ("resource", parts[0].to_string()),
                };
                ("api", resource_type, resource_id)
            } else {
                ("api", "api", api_path.to_string())
            }
        }
        p if p.starts_with("/api/") => {
            let api_path = p.trim_start_matches("/api/");
            ("api", "api", api_path.to_string())
        }

        // Auth endpoints
        p if p.starts_with("/v1/auth/login") => ("auth", "auth", "login".to_string()),
        p if p.starts_with("/v1/auth/logout") => ("auth", "auth", "logout".to_string()),
        p if p.starts_with("/v1/auth/") => ("auth", "auth", "other".to_string()),
        p if p.starts_with("/auth/") => ("auth", "auth", "legacy".to_string()),

        // Health check
        p if p == "/health" || p == "/healthz" => ("service", "service", "health".to_string()),
        p if p.starts_with("/metrics") => ("service", "service", "metrics".to_string()),

        // Default
        _ => ("service", "service", path.to_string()),
    };

    let hrn = format!(
        "hrn:hodei:{}:{}:global:{}/{}",
        service_type, tenant, resource_type, resource_id
    );

    Hrn::parse(&hrn)
}

/// Extraer ID desde path (ej: /v1/policy-stores/{id}/... -> {id})
fn extract_id_from_path(path: &str, segment_index: usize) -> Option<String> {
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if segments.len() > segment_index {
        Some(segments[segment_index].to_string())
    } else {
        None
    }
}

/// Determinar tipo de servicio desde path
fn service_type_from_path(path: &str) -> &'static str {
    if path.starts_with("/v1/policy-stores") || path.starts_with("/v1/authorize") {
        "verified-permissions"
    } else if path.starts_with("/api/") {
        "api"
    } else if path.starts_with("/v1/auth/") || path.starts_with("/auth/") {
        "auth"
    } else if path == "/health" || path == "/healthz" || path.starts_with("/metrics") {
        "service"
    } else {
        "service"
    }
}

/// Enriquecer evento con metadata de HRN
pub async fn enrich_event_with_hrn(
    event: &mut crate::models::AuditEvent,
    resolver: &Option<Arc<dyn HrnResolver>>,
) -> Result<(), AuditError> {
    if let Some(r) = resolver {
        // Parse HRN
        let hrn = Hrn::parse(&event.hrn)?;

        // Resolve metadata
        let metadata = r.resolve(hrn.as_str())?;

        // Create or get the additional_data map
        let map = event
            .additional_data
            .get_or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));

        // Ensure it's a map/object and get mutable reference
        if let serde_json::Value::Object(ref mut map) = *map {
            map.insert(
                "hrn_display_name".to_string(),
                serde_json::Value::String(metadata.display_name),
            );
            map.insert(
                "hrn_resource_type".to_string(),
                serde_json::Value::String(metadata.resource_type),
            );
            if !metadata.tags.is_empty() {
                map.insert(
                    "hrn_tags".to_string(),
                    serde_json::to_value(&metadata.tags)?,
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hrn_parse_valid() {
        let hrn = Hrn::parse("hrn:hodei:api:tenant-123:global:user/user-456").unwrap();
        assert_eq!(hrn.service(), "api");
        assert_eq!(hrn.tenant(), "tenant-123");
        assert_eq!(hrn.resource_type(), "user");
        assert_eq!(hrn.resource_id(), "user-456");
        assert_eq!(
            hrn.as_str(),
            "hrn:hodei:api:tenant-123:global:user/user-456"
        );
    }

    #[test]
    fn test_hrn_parse_invalid() {
        let result = Hrn::parse("invalid");
        assert!(result.is_err());

        let result = Hrn::parse("hodei:api:tenant-123:global:user/user-456");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_hrn_policy_stores() {
        let hrn = generate_hrn_from_path(
            &http::Method::GET,
            "/v1/policy-stores/default/policies",
            Some("tenant-123"),
        )
        .unwrap();

        assert!(hrn.as_str().contains("verified-permissions"));
        assert!(hrn.as_str().contains("tenant-123"));
        assert!(hrn.as_str().contains("policy-store/default"));
    }

    #[test]
    fn test_generate_hrn_authorize() {
        let hrn = generate_hrn_from_path(&http::Method::POST, "/v1/authorize", Some("tenant-123"))
            .unwrap();

        assert!(hrn.as_str().contains("verified-permissions"));
        assert!(hrn.as_str().contains("authorization"));
    }

    #[test]
    fn test_generate_hrn_api_users() {
        let hrn =
            generate_hrn_from_path(&http::Method::GET, "/api/v1/users/456", Some("tenant-123"))
                .unwrap();

        assert!(hrn.as_str().contains("api"));
        assert!(hrn.as_str().contains("user/456"));
    }

    #[test]
    fn test_generate_hrn_auth() {
        let hrn = generate_hrn_from_path(&http::Method::POST, "/v1/auth/login", Some("tenant-123"))
            .unwrap();

        assert!(hrn.as_str().contains("auth"));
        assert!(hrn.as_str().contains("login"));
    }

    #[test]
    fn test_generate_hrn_health() {
        let hrn =
            generate_hrn_from_path(&http::Method::GET, "/health", Some("tenant-123")).unwrap();

        assert!(hrn.as_str().contains("service"));
        assert!(hrn.as_str().contains("health"));
    }

    #[test]
    fn test_extract_id_from_path() {
        assert_eq!(
            extract_id_from_path("/v1/policy-stores/store-123/policies", 2),
            Some("store-123".to_string())
        );
        assert_eq!(
            extract_id_from_path("/api/v1/users/user-456", 3),
            Some("user-456".to_string())
        );
        assert_eq!(extract_id_from_path("/short/path", 5), None);
    }

    #[test]
    fn test_service_type_from_path() {
        assert_eq!(
            service_type_from_path("/v1/policy-stores"),
            "verified-permissions"
        );
        assert_eq!(
            service_type_from_path("/v1/authorize"),
            "verified-permissions"
        );
        assert_eq!(service_type_from_path("/api/test"), "api");
        assert_eq!(service_type_from_path("/v1/auth/login"), "auth");
        assert_eq!(service_type_from_path("/health"), "service");
        assert_eq!(service_type_from_path("/unknown"), "service");
    }
}
