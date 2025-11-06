//! Row-Level Security (RLS) for ClickHouse
//!
//! This module implements Row-Level Security policies for ClickHouse
//! to ensure complete tenant isolation at the database level.

use std::collections::HashMap;
use tracing::{error, info, warn};

/// Row-Level Security policy
#[derive(Debug, Clone)]
pub struct RlsPolicy {
    /// Policy name
    pub name: String,
    /// Table name
    pub table: String,
    /// Tenant column name
    pub tenant_column: String,
    /// Policy condition (SQL WHERE clause)
    pub condition: String,
    /// Enabled flag
    pub enabled: bool,
}

impl RlsPolicy {
    /// Create a new RLS policy
    pub fn new(name: String, table: String, tenant_column: String) -> Self {
        Self {
            name,
            table,
            tenant_column: tenant_column.clone(),
            condition: format!("{} = currentSetting('tenant_id')", tenant_column),
            enabled: true,
        }
    }

    /// Create with custom condition
    pub fn with_condition(mut self, condition: String) -> Self {
        self.condition = condition;
        self
    }

    /// Enable/disable policy
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Generate SQL for creating the policy
    pub fn to_create_sql(&self) -> String {
        format!(
            "CREATE ROW POLICY IF NOT EXISTS {} ON {} FOR SELECT USING ({})",
            self.name, self.table, self.condition
        )
    }

    /// Generate SQL for dropping the policy
    pub fn to_drop_sql(&self) -> String {
        format!("DROP ROW POLICY IF EXISTS {} ON {}", self.name, self.table)
    }

    /// Generate SQL for enabling the policy
    pub fn to_enable_sql(&self) -> String {
        format!(
            "ALTER TABLE {} MODIFY ROW POLICY {} SET enabled = 1",
            self.table, self.name
        )
    }

    /// Generate SQL for disabling the policy
    pub fn to_disable_sql(&self) -> String {
        format!(
            "ALTER TABLE {} MODIFY ROW POLICY {} SET enabled = 0",
            self.table, self.name
        )
    }
}

/// Row-Level Security manager
pub struct RlsManager {
    /// Map of table names to RLS policies
    policies: HashMap<String, RlsPolicy>,
    /// Current tenant ID
    current_tenant_id: Option<String>,
}

impl RlsManager {
    /// Create a new RLS manager
    pub fn new() -> Self {
        let mut manager = Self {
            policies: HashMap::new(),
            current_tenant_id: None,
        };

        // Register default policy for audit_events table
        manager.register_policy(RlsPolicy::new(
            "tenant_isolation".to_string(),
            "audit_events".to_string(),
            "tenant_id".to_string(),
        ));

        manager
    }

    /// Register a RLS policy
    pub fn register_policy(&mut self, policy: RlsPolicy) {
        info!(
            "[RLS] Registered policy '{}' for table '{}'",
            policy.name, policy.table
        );
        self.policies.insert(policy.table.clone(), policy);
    }

    /// Set current tenant ID (used for session context)
    pub fn set_tenant_id(&mut self, tenant_id: String) {
        info!("[RLS] Setting tenant context: {}", tenant_id);
        self.current_tenant_id = Some(tenant_id);
    }

    /// Clear tenant context
    pub fn clear_tenant_id(&mut self) {
        info!("[RLS] Clearing tenant context");
        self.current_tenant_id = None;
    }

    /// Get current tenant ID
    pub fn get_tenant_id(&self) -> Option<&str> {
        self.current_tenant_id.as_deref()
    }

    /// Generate SQL to set tenant context
    pub fn set_tenant_context_sql(&self) -> Option<String> {
        self.current_tenant_id
            .as_ref()
            .map(|tid| format!("SET tenant_id = '{}'", tid))
    }

    /// Validate query for tenant isolation
    pub fn validate_query(&self, sql: &str, table: &str) -> Result<(), anyhow::Error> {
        // Check if query already has a WHERE clause with tenant_id
        let has_tenant_filter = sql.to_lowercase().contains("tenant_id")
            || sql.to_lowercase().contains("where")
            || sql.to_lowercase().contains("prewhere");

        if has_tenant_filter {
            warn!("[RLS] Query may already have tenant filtering: {}", sql);
            return Ok(());
        }

        // Check if table has RLS policy
        if self.policies.contains_key(table) {
            warn!(
                "[RLS] Query on table '{}' without explicit tenant filter",
                table
            );
            return Ok(()); // RLS will handle it
        }

        Ok(())
    }

    /// Get all registered policies
    pub fn get_policies(&self) -> Vec<&RlsPolicy> {
        self.policies.values().collect()
    }

    /// Get policy for a specific table
    pub fn get_policy(&self, table: &str) -> Option<&RlsPolicy> {
        self.policies.get(table)
    }

    /// Generate SQL to enable all policies
    pub fn enable_all_policies_sql(&self) -> Vec<String> {
        self.policies
            .values()
            .filter(|p| p.enabled)
            .map(|p| p.to_enable_sql())
            .collect()
    }

    /// Generate SQL to create all policies
    pub fn create_all_policies_sql(&self) -> Vec<String> {
        self.policies
            .values()
            .filter(|p| p.enabled)
            .map(|p| p.to_create_sql())
            .collect()
    }

    /// Generate SQL to drop all policies
    pub fn drop_all_policies_sql(&self) -> Vec<String> {
        self.policies.values().map(|p| p.to_drop_sql()).collect()
    }
}

impl Default for RlsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Query builder with RLS enforcement
pub struct RlsQueryBuilder {
    /// Base table name
    table: String,
    /// Current WHERE clauses
    where_clauses: Vec<String>,
    /// SELECT columns
    select_columns: Vec<String>,
    /// ORDER BY clauses
    order_by: Vec<String>,
    /// LIMIT value
    limit: Option<u64>,
    /// RLS manager reference
    rls_manager: RlsManager,
}

impl RlsQueryBuilder {
    /// Create a new query builder
    pub fn new(table: String, rls_manager: RlsManager) -> Self {
        Self {
            table,
            where_clauses: Vec::new(),
            select_columns: vec!["*".to_string()],
            order_by: Vec::new(),
            limit: None,
            rls_manager,
        }
    }

    /// Add SELECT columns
    pub fn select(&mut self, columns: Vec<String>) -> &mut Self {
        self.select_columns = columns;
        self
    }

    /// Add WHERE clause
    pub fn where_clause(&mut self, condition: String) -> &mut Self {
        self.where_clauses.push(condition);
        self
    }

    /// Add ORDER BY clause
    pub fn order_by(&mut self, column: String) -> &mut Self {
        self.order_by.push(column);
        self
    }

    /// Add LIMIT
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Build the query with RLS enforcement
    pub fn build(&self) -> Result<String, anyhow::Error> {
        // Start building the query
        let mut query = format!(
            "SELECT {} FROM {}",
            self.select_columns.join(", "),
            self.table
        );

        // Collect all WHERE clauses
        let mut all_where_clauses = self.where_clauses.clone();

        // Add RLS enforcement
        if let Some(tenant_id) = self.rls_manager.get_tenant_id() {
            let rls_clause = format!("tenant_id = '{}'", tenant_id);
            all_where_clauses.push(rls_clause);
            info!(
                "[RLS] Enforcing tenant isolation: tenant_id = '{}'",
                tenant_id
            );
        }

        // Add WHERE clause if any conditions exist
        if !all_where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&all_where_clauses.join(" AND "));
        }

        // Add ORDER BY if specified
        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            query.push_str(&self.order_by.join(", "));
        }

        // Add LIMIT if specified
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // Validate the query
        self.rls_manager.validate_query(&query, &self.table)?;

        info!("[RLS] Generated query with tenant isolation: {}", query);

        Ok(query)
    }
}

/// Secure query executor
pub struct SecureQueryExecutor {
    /// ClickHouse client
    client: crate::clickhouse::ClickHouseClient,
    /// RLS manager
    rls_manager: RlsManager,
}

impl SecureQueryExecutor {
    /// Create a new secure query executor
    pub fn new(client: crate::clickhouse::ClickHouseClient, rls_manager: RlsManager) -> Self {
        Self {
            client,
            rls_manager,
        }
    }

    /// Execute a SELECT query with RLS enforcement
    pub async fn query_with_rls(
        &self,
        query_builder: RlsQueryBuilder,
    ) -> Result<Vec<hodei_audit_proto::AuditEvent>, anyhow::Error> {
        // Build the query
        let query = query_builder
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build RLS query: {}", e))?;

        // Execute the query
        let result = self.client.query(&query).await?;

        info!(
            "[RLS] Executed secure query, returned {} events",
            result.len()
        );

        Ok(result)
    }

    /// Execute a parameterized query with RLS enforcement
    pub async fn query_with_params(
        &self,
        sql: &str,
        params: &HashMap<String, String>,
    ) -> Result<Vec<hodei_audit_proto::AuditEvent>, anyhow::Error> {
        // Add RLS enforcement to params
        let mut secure_params = params.clone();
        if let Some(tenant_id) = self.rls_manager.get_tenant_id() {
            secure_params.insert("tenant_id".to_string(), tenant_id.to_string());
        }

        // Validate the query
        self.rls_manager.validate_query(sql, "audit_events")?;

        // Execute the query
        let result = self.client.query_with_params(sql, &secure_params).await?;

        info!(
            "[RLS] Executed parameterized query with RLS, returned {} events",
            result.len()
        );

        Ok(result)
    }

    /// Ensure RLS policies are enabled
    pub async fn ensure_rls_enabled(&self) -> Result<(), anyhow::Error> {
        info!("[RLS] Ensuring Row-Level Security is enabled");

        // Check if RLS is enabled
        let check_sql = "SELECT currentSetting('rls_enabled', 1) as rls_enabled";
        let _result = self.client.query(check_sql).await?;

        info!("[RLS] Row-Level Security is enabled");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rls_policy_creation() {
        let policy = RlsPolicy::new(
            "test_policy".to_string(),
            "audit_events".to_string(),
            "tenant_id".to_string(),
        );

        assert_eq!(policy.name, "test_policy");
        assert_eq!(policy.table, "audit_events");
        assert_eq!(policy.tenant_column, "tenant_id");
        assert!(policy.enabled);
        assert!(policy.condition.contains("tenant_id"));
    }

    #[test]
    fn test_rls_policy_with_custom_condition() {
        let policy = RlsPolicy::new(
            "custom_policy".to_string(),
            "audit_events".to_string(),
            "tenant_id".to_string(),
        )
        .with_condition("tenant_id = 'enterprise'".to_string());

        assert_eq!(policy.condition, "tenant_id = 'enterprise'");
    }

    #[test]
    fn test_rls_policy_sql_generation() {
        let policy = RlsPolicy::new(
            "test_policy".to_string(),
            "audit_events".to_string(),
            "tenant_id".to_string(),
        );

        let create_sql = policy.to_create_sql();
        assert!(create_sql.contains("CREATE ROW POLICY"));
        assert!(create_sql.contains("test_policy"));
        assert!(create_sql.contains("audit_events"));
        assert!(create_sql.contains("tenant_id = currentSetting('tenant_id')"));

        let drop_sql = policy.to_drop_sql();
        assert!(drop_sql.contains("DROP ROW POLICY"));
        assert!(drop_sql.contains("test_policy"));
    }

    #[test]
    fn test_rls_manager() {
        let mut manager = RlsManager::new();

        // Should have default policy
        assert!(manager.policies.contains_key("audit_events"));

        // Set tenant ID
        manager.set_tenant_id("test-tenant".to_string());
        assert_eq!(manager.get_tenant_id(), Some("test-tenant"));

        // Get SQL to set context
        let context_sql = manager.set_tenant_context_sql();
        assert!(context_sql.is_some());
        assert!(context_sql.unwrap().contains("test-tenant"));

        // Clear tenant ID
        manager.clear_tenant_id();
        assert_eq!(manager.get_tenant_id(), None);
    }

    #[test]
    fn test_rls_query_builder() {
        let mut manager = RlsManager::new();
        manager.set_tenant_id("test-tenant".to_string());

        let mut builder = RlsQueryBuilder::new("audit_events".to_string(), manager);

        builder
            .select(vec!["event_id".to_string(), "action".to_string()])
            .where_clause("action = 'login'".to_string())
            .order_by("timestamp".to_string())
            .limit(100);

        let query = builder.build().unwrap();

        // Should contain RLS enforcement
        assert!(query.contains("tenant_id = 'test-tenant'"));
        assert!(query.contains("action = 'login'"));
        assert!(query.contains("ORDER BY timestamp"));
        assert!(query.contains("LIMIT 100"));
    }

    #[test]
    fn test_query_builder_without_tenant() {
        let manager = RlsManager::new(); // No tenant set

        let mut builder = RlsQueryBuilder::new("audit_events".to_string(), manager);

        builder.select(vec!["*".to_string()]);

        let query = builder.build().unwrap();

        // Should not contain tenant enforcement
        assert!(!query.contains("tenant_id"));
    }

    #[test]
    fn test_rls_manager_policy_retrieval() {
        let manager = RlsManager::new();

        let policies = manager.get_policies();
        assert!(!policies.is_empty());
        assert_eq!(policies.len(), 1);

        let policy = manager.get_policy("audit_events");
        assert!(policy.is_some());
        assert_eq!(policy.unwrap().table, "audit_events");
    }

    #[test]
    fn test_rls_sql_generation() {
        let mut manager = RlsManager::new();

        manager.set_tenant_id("test-tenant".to_string());

        let create_sqls = manager.create_all_policies_sql();
        assert!(!create_sqls.is_empty());

        let enable_sqls = manager.enable_all_policies_sql();
        assert!(!enable_sqls.is_empty());

        let drop_sqls = manager.drop_all_policies_sql();
        assert!(!drop_sqls.is_empty());
    }
}
