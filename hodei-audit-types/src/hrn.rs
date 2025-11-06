//! Hodei Resource Name (HRN) system
//!
//! HRN provides unique hierarchical identifiers for all resources
//! in the Hodei ecosystem, inspired by AWS ARNs but designed for multi-tenancy

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Hodei Resource Name - unique identifier for resources
/// Format: hrn:partition:service:tenant:region:resource-type/resource-path
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hrn {
    pub partition: String,      // "hodei"
    pub service: String,        // "verified-permissions", "api", etc.
    pub tenant_id: String,      // Tenant identifier
    pub region: Option<String>, // "eu-west-1", "global", etc.
    pub resource_type: String,  // "policy-store", "api", etc.
    pub resource_path: String,  // "default", "user-123", etc.
}

impl Hrn {
    /// Create a new HRN
    pub fn new(
        partition: String,
        service: String,
        tenant_id: String,
        region: Option<String>,
        resource_type: String,
        resource_path: String,
    ) -> Self {
        Self {
            partition,
            service,
            tenant_id,
            region,
            resource_type,
            resource_path,
        }
    }

    /// Parse HRN from string
    pub fn parse<S: Into<String>>(s: S) -> Result<Self, HrnError> {
        let s = s.into();
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() < 6 {
            return Err(HrnError::InvalidFormat {
                input: s.clone(),
                reason: "Expected at least 6 parts (hrn:partition:service:tenant:region:type/path)".to_string(),
            });
        }

        if parts[0] != "hrn" {
            return Err(HrnError::InvalidFormat {
                input: s.clone(),
                reason: "Must start with 'hrn'".to_string(),
            });
        }

        // Parse resource path (may contain colons after resource type)
        let resource_str = parts[5..].join(":");
        let resource_parts: Vec<&str> = resource_str.split('/').collect();
        let resource_type = resource_parts[0].to_string();
        let resource_path = if resource_parts.len() > 1 {
            resource_parts[1..].join("/")
        } else {
            "".to_string()
        };

        Ok(Self {
            partition: parts[1].to_string(),
            service: parts[2].to_string(),
            tenant_id: parts[3].to_string(),
            region: match parts[4] {
                "global" | "" => None,
                r => Some(r.to_string()),
            },
            resource_type,
            resource_path,
        })
    }

    /// Convert HRN to string
    pub fn to_string(&self) -> String {
        format!(
            "hrn:{}:{}:{}:{}:{}",
            self.partition,
            self.service,
            self.tenant_id,
            self.region.as_deref().unwrap_or("global"),
            self.resource_type
        ) + "/" + &self.resource_path
    }

    /// Get parent HRN (remove last path component)
    pub fn parent(&self) -> Option<Self> {
        let mut parts: Vec<&str> = self.resource_path.split('/').collect();
        parts.pop()?;

        if parts.is_empty() {
            return None;
        }

        Some(Self {
            partition: self.partition.clone(),
            service: self.service.clone(),
            tenant_id: self.tenant_id.clone(),
            region: self.region.clone(),
            resource_type: self.resource_type.clone(),
            resource_path: parts.join("/"),
        })
    }

    /// Check if this HRN is a child of another
    pub fn is_child_of(&self, parent: &Self) -> bool {
        self.tenant_id == parent.tenant_id
            && self.service == parent.service
            && self.resource_path.starts_with(&parent.resource_path)
    }
}

impl FromStr for Hrn {
    type Err = HrnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Hrn::parse(s)
    }
}

impl std::fmt::Display for Hrn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// HRN-related errors
#[derive(thiserror::Error, Debug)]
pub enum HrnError {
    #[error("Invalid HRN format: {reason}")]
    InvalidFormat { input: String, reason: String },

    #[error("Invalid HRN: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HrnMetadata {
    pub hrn: Hrn,
    pub display_name: String,
    pub description: Option<String>,
    pub tags: std::collections::BTreeMap<String, String>,
    pub owner: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait::async_trait]
pub trait HrnResolver: Send + Sync {
    async fn resolve(&self, hrn: &Hrn) -> Result<HrnMetadata, HrnError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hrn_parse_and_format() {
        let hrn_str = "hrn:hodei:verified-permissions:tenant-123:global:policy-store/default";
        let hrn = Hrn::parse(hrn_str).unwrap();

        assert_eq!(hrn.partition, "hodei");
        assert_eq!(hrn.service, "verified-permissions");
        assert_eq!(hrn.tenant_id, "tenant-123");
        assert_eq!(hrn.region, None);
        assert_eq!(hrn.resource_type, "policy-store");
        assert_eq!(hrn.resource_path, "default");

        assert_eq!(hrn.to_string(), hrn_str);
    }

    #[test]
    fn test_hrn_parent() {
        let hrn = Hrn::parse("hrn:hodei:verified-permissions:tenant-123:global:policy/default/child").unwrap();
        let parent = hrn.parent().expect("Expected parent to exist");

        assert_eq!(parent.resource_path, "default");
        assert_eq!(parent.resource_type, "policy");
    }

    #[test]
    fn test_hrn_is_child_of() {
        let parent = Hrn::parse("hrn:hodei:verified-permissions:tenant-123:global:policy-store").unwrap();
        let child = Hrn::parse("hrn:hodei:verified-permissions:tenant-123:global:policy-store/default").unwrap();
        let other = Hrn::parse("hrn:hodei:api:tenant-123:global:api/test").unwrap();

        assert!(child.is_child_of(&parent));
        assert!(!other.is_child_of(&parent));
    }
}
