//! Application configuration

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub audit_service_url: String,
    pub audit_enabled: bool,
    pub audit_tenant_id: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()?;
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://petclinic:petclinic@localhost:5432/petclinic".to_string()
        });
        let audit_service_url = std::env::var("HODEI_AUDIT_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:50052".to_string());
        let audit_enabled = std::env::var("HODEI_AUDIT_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse()?;
        let audit_tenant_id = std::env::var("HODEI_AUDIT_TENANT_ID")
            .unwrap_or_else(|_| "tenant-petclinic".to_string());

        Ok(AppConfig {
            port,
            database_url,
            audit_service_url,
            audit_enabled,
            audit_tenant_id,
        })
    }
}
