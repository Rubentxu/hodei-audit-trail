//! Tipos comunes para el SDK de auditoría

/// Consulta de eventos de auditoría
#[derive(Debug, Clone)]
pub struct AuditQuery {
    /// HRN del recurso (opcional)
    pub hrn: Option<String>,
    /// Tenant ID (opcional)
    pub tenant_id: Option<String>,
    /// User ID (opcional)
    pub user_id: Option<String>,
    /// Timestamp de inicio (opcional)
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp de fin (opcional)
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Límite de resultados
    pub limit: u64,
    /// Offset para paginación
    pub offset: u64,
}

impl Default for AuditQuery {
    fn default() -> Self {
        Self {
            hrn: None,
            tenant_id: None,
            user_id: None,
            start_time: None,
            end_time: None,
            limit: 100,
            offset: 0,
        }
    }
}
