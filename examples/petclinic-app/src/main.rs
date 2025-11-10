//! Main application entry point

use axum::Router;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod domain;
mod infrastructure;
mod presentation;
mod config;

use config::AppConfig;
use presentation::controllers::AppState;
use infrastructure::repositories::{
    SqlxOwnerRepository, SqlxPetRepository, SqlxVisitRepository, SqlxVetRepository,
    SqlxPetTypeRepository,
};
use application::services::{
    OwnerApplicationService, PetApplicationService, VisitApplicationService,
    VetApplicationService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "petclinic_app=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::load()?;

    // Create database connection pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Initialize repositories
    let owner_repo = SqlxOwnerRepository::new(pool.clone());
    let pet_repo = SqlxPetRepository::new(pool.clone());
    let visit_repo = SqlxVisitRepository::new(pool.clone());
    let vet_repo = SqlxVetRepository::new(pool.clone());
    let pet_type_repo = SqlxPetTypeRepository::new(pool.clone());

    // Initialize services
    let owner_service = Arc::new(OwnerApplicationService {
        owner_repo: owner_repo.clone(),
        pet_repo: pet_repo.clone(),
        visit_repo: visit_repo.clone(),
    });

    let pet_service = Arc::new(PetApplicationService {
        pet_repo: pet_repo.clone(),
        visit_repo: visit_repo.clone(),
        owner_repo: owner_repo.clone(),
    });

    let visit_service = Arc::new(VisitApplicationService {
        visit_repo: visit_repo.clone(),
        pet_repo: pet_repo.clone(),
    });

    let vet_service = Arc::new(VetApplicationService {
        vet_repo: vet_repo.clone(),
        specialty_repo: pet_type_repo.clone(), // Using pet_type_repo as specialty_repo for now
    });

    // Create app state
    let state = AppState {
        owner_service,
        pet_service,
        visit_service,
        vet_service,
    };

    // Create Axum router
    let app = presentation::controllers::create_routes(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", config.port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
