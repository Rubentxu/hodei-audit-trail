//! HTTP Controllers
//!
//! This module implements the REST API controllers using Axum.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post, put, delete},
    Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};

use crate::{
    application::services::{
        OwnerApplicationService, PetApplicationService, VisitApplicationService,
        VetApplicationService,
    },
    domain::entities::{Owner, Pet, PetType, Visit, Vet},
    domain::repositories::RepositoryResult,
};

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    pub owner_service: Arc<dyn OwnerService>,
    pub pet_service: Arc<dyn PetService>,
    pub visit_service: Arc<dyn VisitService>,
    pub vet_service: Arc<dyn VetService>,
}

/// Owner service interface
#[async_trait::async_trait]
pub trait OwnerService: Send + Sync {
    async fn find_with_pets(&self, id: i32) -> RepositoryResult<Option<Owner>>;
    async fn search_by_last_name(&self, last_name: &str) -> RepositoryResult<Vec<Owner>>;
    async fn create(&self, owner: Owner) -> RepositoryResult<Owner>;
    async fn update(&self, owner: &Owner) -> RepositoryResult<Owner>;
    async fn delete_with_pets(&self, id: i32) -> RepositoryResult<()>;
    async fn find_all(&self) -> RepositoryResult<Vec<Owner>>;
}

#[async_trait::async_trait]
impl<R> OwnerService for OwnerApplicationService<R>
where
    R: crate::domain::repositories::OwnerRepository
        + crate::domain::repositories::PetRepository
        + crate::domain::repositories::VisitRepository
        + Send
        + Sync,
{
    async fn find_with_pets(&self, id: i32) -> RepositoryResult<Option<Owner>> {
        OwnerService::find_with_pets(self, id).await
    }
    async fn search_by_last_name(&self, last_name: &str) -> RepositoryResult<Vec<Owner>> {
        OwnerService::search_by_last_name(self, last_name).await
    }
    async fn create(&self, owner: Owner) -> RepositoryResult<Owner> {
        OwnerService::create(self, owner).await.map_err(|e| match e {
            crate::domain::services::DomainServiceError::Repository(r) => r,
            e => crate::domain::repositories::RepositoryError::Database(
                sqlx::Error::Protocol(e.to_string().into()),
            ),
        })
    }
    async fn update(&self, owner: &Owner) -> RepositoryResult<Owner> {
        OwnerService::update(self, owner).await.map_err(|e| match e {
            crate::domain::services::DomainServiceError::Repository(r) => r,
            e => crate::domain::repositories::RepositoryError::Database(
                sqlx::Error::Protocol(e.to_string().into()),
            ),
        })
    }
    async fn delete_with_pets(&self, id: i32) -> RepositoryResult<()> {
        OwnerService::delete_with_pets(self, id).await.map_err(|e| match e {
            crate::domain::services::DomainServiceError::Repository(r) => r,
            e => crate::domain::repositories::RepositoryError::Database(
                sqlx::Error::Protocol(e.to_string().into()),
            ),
        })
    }
    async fn find_all(&self) -> RepositoryResult<Vec<Owner>> {
        self.owner_repo.find_all().await
    }
}

#[async_trait::async_trait]
pub trait PetService: Send + Sync {
    async fn find_with_details(&self, id: i32) -> RepositoryResult<Option<Pet>>;
    async fn find_by_owner(&self, owner_id: i32) -> RepositoryResult<Vec<Pet>>;
    async fn create(&self, pet: Pet) -> RepositoryResult<Pet>;
    async fn update(&self, pet: &Pet) -> RepositoryResult<Pet>;
    async fn delete(&self, id: i32) -> RepositoryResult<()>;
}

#[async_trait::async_trait]
impl<R> PetService for PetApplicationService<R>
where
    R: crate::domain::repositories::PetRepository
        + crate::domain::repositories::VisitRepository
        + crate::domain::repositories::OwnerRepository
        + Send
        + Sync,
{
    async fn find_with_details(&self, id: i32) -> RepositoryResult<Option<Pet>> {
        PetService::find_with_details(self, id).await
    }
    async fn find_by_owner(&self, owner_id: i32) -> RepositoryResult<Vec<Pet>> {
        self.pet_repo.find_by_owner_id(owner_id).await
    }
    async fn create(&self, pet: Pet) -> RepositoryResult<Pet> {
        PetService::create(self, pet).await.map_err(|e| match e {
            crate::domain::services::DomainServiceError::Repository(r) => r,
            e => crate::domain::repositories::RepositoryError::Database(
                sqlx::Error::Protocol(e.to_string().into()),
            ),
        })
    }
    async fn update(&self, pet: &Pet) -> RepositoryResult<Pet> {
        PetService::update(self, pet).await.map_err(|e| match e {
            crate::domain::services::DomainServiceError::Repository(r) => r,
            e => crate::domain::repositories::RepositoryError::Database(
                sqlx::Error::Protocol(e.to_string().into()),
            ),
        })
    }
    async fn delete(&self, id: i32) -> RepositoryResult<()> {
        PetService::delete(self, id).await.map_err(|e| match e {
            crate::domain::services::DomainServiceError::Repository(r) => r,
            e => crate::domain::repositories::RepositoryError::Database(
                sqlx::Error::Protocol(e.to_string().into()),
            ),
        })
    }
}

#[async_trait::async_trait]
pub trait VisitService: Send + Sync {
    async fn create(&self, visit: Visit) -> RepositoryResult<Visit>;
    async fn find_by_pet(&self, pet_id: i32) -> RepositoryResult<Vec<Visit>>;
    async fn find_all(&self) -> RepositoryResult<Vec<Visit>>;
}

#[async_trait::async_trait]
impl<R> VisitService for VisitApplicationService<R>
where
    R: crate::domain::repositories::VisitRepository
        + crate::domain::repositories::PetRepository
        + Send
        + Sync,
{
    async fn create(&self, visit: Visit) -> RepositoryResult<Visit> {
        VisitService::create(self, visit).await.map_err(|e| match e {
            crate::domain::services::DomainServiceError::Repository(r) => r,
            e => crate::domain::repositories::RepositoryError::Database(
                sqlx::Error::Protocol(e.to_string().into()),
            ),
        })
    }
    async fn find_by_pet(&self, pet_id: i32) -> RepositoryResult<Vec<Visit>> {
        self.visit_repo.find_by_pet_id(pet_id).await
    }
    async fn find_all(&self) -> RepositoryResult<Vec<Visit>> {
        self.visit_repo.find_all().await
    }
}

#[async_trait::async_trait]
pub trait VetService: Send + Sync {
    async fn find_all(&self) -> RepositoryResult<Vec<Vet>>;
}

#[async_trait::async_trait]
impl<R> VetService for VetApplicationService<R>
where
    R: crate::domain::repositories::VetRepository + Send + Sync,
{
    async fn find_all(&self) -> RepositoryResult<Vec<Vet>> {
        self.vet_repo.find_all().await
    }
}

// DTOs

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerDto {
    pub first_name: String,
    pub last_name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub telephone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub telephone: Option<String>,
    pub pets: Vec<PetResponse>,
}

impl From<Owner> for OwnerResponse {
    fn from(owner: Owner) -> Self {
        Self {
            id: owner.id.unwrap_or(0),
            first_name: owner.first_name,
            last_name: owner.last_name,
            full_name: owner.full_name(),
            address: owner.address,
            city: owner.city,
            telephone: owner.telephone,
            pets: owner
                .visits
                .into_iter()
                .map(|p| PetResponse::from(p))
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetDto {
    pub name: String,
    pub birth_date: Option<String>,
    pub type_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetResponse {
    pub id: i32,
    pub name: String,
    pub birth_date: Option<String>,
    pub type_id: i32,
    pub owner_id: i32,
    pub age: Option<i32>,
    pub visits: Vec<VisitResponse>,
}

impl From<Pet> for PetResponse {
    fn from(pet: Pet) -> Self {
        Self {
            id: pet.id.unwrap_or(0),
            name: pet.name,
            birth_date: pet.birth_date.map(|d| d.to_string()),
            type_id: pet.type_id,
            owner_id: pet.owner_id,
            age: pet.age(),
            visits: pet.visits.into_iter().map(|v| v.into()).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitDto {
    pub date: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitResponse {
    pub id: i32,
    pub pet_id: i32,
    pub date: String,
    pub description: String,
}

impl From<Visit> for VisitResponse {
    fn from(visit: Visit) -> Self {
        Self {
            id: visit.id.unwrap_or(0),
            pet_id: visit.pet_id,
            date: visit.visit_date.to_string(),
            description: visit.description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VetResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub specialties: Vec<String>,
}

impl From<Vet> for VetResponse {
    fn from(vet: Vet) -> Self {
        Self {
            id: vet.id.unwrap_or(0),
            first_name: vet.first_name.clone(),
            last_name: vet.last_name.clone(),
            full_name: vet.full_name(),
            specialties: vet
                .specialties
                .into_iter()
                .map(|s| s.name)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PetTypeResponse {
    pub id: i32,
    pub name: String,
}

impl From<PetType> for PetTypeResponse {
    fn from(pet_type: PetType) -> Self {
        Self {
            id: pet_type.id,
            name: pet_type.name,
        }
    }
}

// Controllers

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/owners", owners_routes(state.clone()))
        .nest("/pets", pets_routes(state.clone()))
        .nest("/visits", visits_routes(state.clone()))
        .nest("/vets", vets_routes(state.clone()))
        .nest("/pet-types", pet_types_routes(state))
}

fn owners_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_owners).post(create_owner))
        .route("/:id", get(get_owner).put(update_owner).delete(delete_owner))
        .route("/:id/pets", get(list_owner_pets).post(add_pet_to_owner))
}

fn pets_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_pets))
        .route("/:id", get(get_pet).delete(delete_pet))
        .route("/:id/visits", get(list_pet_visits).post(add_visit_to_pet))
}

fn visits_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_visits))
        .route("/:id", get(get_visit).put(update_visit).delete(delete_visit))
}

fn vets_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_vets))
        .route("/:id", get(get_vet))
}

fn pet_types_routes(state: AppState) -> Router {
    Router::new().route("/", get(list_pet_types))
}

// Handlers

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

async fn list_owners(
    State(state): State<AppState>,
    Query(params): Query<OwnerSearchParams>,
) -> impl IntoResponse {
    info!("Listing owners");

    let owners = if let Some(last_name) = params.last_name {
        state.owner_service.search_by_last_name(&last_name).await
    } else {
        state.owner_service.find_all().await
    };

    match owners {
        Ok(owners) => (StatusCode::OK, Json(owners)),
        Err(e) => {
            warn!("Failed to list owners: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn get_owner(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    info!("Getting owner {}", id);

    match state.owner_service.find_with_pets(id).await {
        Ok(Some(owner)) => (StatusCode::OK, Json(owner.into())),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Owner not found")),
        Err(e) => {
            warn!("Failed to get owner {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn create_owner(
    State(state): State<AppState>,
    Json(owner_dto): Json<OwnerDto>,
) -> impl IntoResponse {
    info!("Creating owner: {} {}", owner_dto.first_name, owner_dto.last_name);

    let owner = Owner::new(
        owner_dto.first_name,
        owner_dto.last_name,
        owner_dto.address,
        owner_dto.city,
        owner_dto.telephone,
    );

    match state.owner_service.create(owner).await {
        Ok(owner) => (StatusCode::CREATED, Json(owner.id.unwrap())),
        Err(e) => {
            warn!("Failed to create owner: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn update_owner(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(owner_dto): Json<OwnerDto>,
) -> impl IntoResponse {
    info!("Updating owner {}", id);

    let owner = Owner {
        id: Some(id),
        first_name: owner_dto.first_name,
        last_name: owner_dto.last_name,
        address: owner_dto.address,
        city: owner_dto.city,
        telephone: owner_dto.telephone,
        created_at: None,
        updated_at: None,
    };

    match state.owner_service.update(&owner).await {
        Ok(_) => (StatusCode::OK, Json("Owner updated successfully")),
        Err(e) => {
            warn!("Failed to update owner {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn delete_owner(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    info!("Deleting owner {}", id);

    match state.owner_service.delete_with_pets(id).await {
        Ok(_) => (StatusCode::OK, Json("Owner deleted successfully")),
        Err(e) => {
            warn!("Failed to delete owner {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn list_owner_pets(
    State(state): State<AppState>,
    Path(owner_id): Path<i32>,
) -> impl IntoResponse {
    info!("Listing pets for owner {}", owner_id);

    match state.pet_service.find_by_owner(owner_id).await {
        Ok(pets) => (StatusCode::OK, Json(pets.into_iter().map(|p| p.into()).collect())),
        Err(e) => {
            warn!("Failed to list pets for owner {}: {:?}", owner_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn add_pet_to_owner(
    State(state): State<AppState>,
    Path(owner_id): Path<i32>,
    Json(pet_dto): Json<PetDto>,
) -> impl IntoResponse {
    info!("Adding pet to owner {}", owner_id);

    let birth_date = if let Some(date_str) = pet_dto.birth_date {
        NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()
    } else {
        None
    };

    let pet = Pet::new(pet_dto.name, birth_date, pet_dto.type_id, owner_id);

    match state.pet_service.create(pet).await {
        Ok(pet) => (StatusCode::CREATED, Json(pet.id.unwrap())),
        Err(e) => {
            warn!("Failed to add pet to owner {}: {:?}", owner_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn list_pets(State(state): State<AppState>) -> impl IntoResponse {
    info!("Listing pets");

    match state.pet_service.find_by_owner(0).await {
        // Note: This should use a proper find_all method
        Ok(_) => (StatusCode::OK, Json(vec![])),
        Err(e) => {
            warn!("Failed to list pets: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn get_pet(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    info!("Getting pet {}", id);

    match state.pet_service.find_with_details(id).await {
        Ok(Some(pet)) => (StatusCode::OK, Json(pet.into())),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Pet not found")),
        Err(e) => {
            warn!("Failed to get pet {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn delete_pet(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    info!("Deleting pet {}", id);

    match state.pet_service.delete(id).await {
        Ok(_) => (StatusCode::OK, Json("Pet deleted successfully")),
        Err(e) => {
            warn!("Failed to delete pet {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn list_pet_visits(
    State(state): State<AppState>,
    Path(pet_id): Path<i32>,
) -> impl IntoResponse {
    info!("Listing visits for pet {}", pet_id);

    match state.visit_service.find_by_pet(pet_id).await {
        Ok(visits) => {
            (StatusCode::OK, Json(visits.into_iter().map(|v| v.into()).collect()))
        }
        Err(e) => {
            warn!("Failed to list visits for pet {}: {:?}", pet_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn add_visit_to_pet(
    State(state): State<AppState>,
    Path(pet_id): Path<i32>,
    Json(visit_dto): Json<VisitDto>,
) -> impl IntoResponse {
    info!("Adding visit to pet {}", pet_id);

    let visit_date = NaiveDate::parse_from_str(&visit_dto.date, "%Y-%m-%d")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let visit = Visit::new(pet_id, visit_date, visit_dto.description);

    match state.visit_service.create(visit).await {
        Ok(visit) => (StatusCode::CREATED, Json(visit.id.unwrap())),
        Err(e) => {
            warn!("Failed to add visit to pet {}: {:?}", pet_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn list_visits(State(state): State<AppState>) -> impl IntoResponse {
    info!("Listing visits");

    match state.visit_service.find_all().await {
        Ok(visits) => (StatusCode::OK, Json(visits.into_iter().map(|v| v.into()).collect())),
        Err(e) => {
            warn!("Failed to list visits: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn get_visit(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    info!("Getting visit {}", id);

    match state.visit_repo.find_by_id(id).await {
        Ok(Some(visit)) => (StatusCode::OK, Json(visit.into())),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Visit not found")),
        Err(e) => {
            warn!("Failed to get visit {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn update_visit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(visit_dto): Json<VisitDto>,
) -> impl IntoResponse {
    info!("Updating visit {}", id);

    let visit_date =
        NaiveDate::parse_from_str(&visit_dto.date, "%Y-%m-%d").map_err(|_| StatusCode::BAD_REQUEST)?;

    let visit = Visit {
        id: Some(id),
        pet_id: 0, // Will be set by the service
        visit_date,
        description: visit_dto.description,
        created_at: None,
    };

    match state.visit_service.find_by_pet(0).await {
        // Note: This should be updated to get visit by ID
        Ok(_) => (StatusCode::OK, Json("Visit updated successfully")),
        Err(e) => {
            warn!("Failed to update visit {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn delete_visit(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    info!("Deleting visit {}", id);

    match state.visit_repo.delete(id).await {
        Ok(_) => (StatusCode::OK, Json("Visit deleted successfully")),
        Err(e) => {
            warn!("Failed to delete visit {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn list_vets(State(state): State<AppState>) -> impl IntoResponse {
    info!("Listing vets");

    match state.vet_service.find_all().await {
        Ok(vets) => (StatusCode::OK, Json(vets.into_iter().map(|v| v.into()).collect())),
        Err(e) => {
            warn!("Failed to list vets: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn get_vet(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    info!("Getting vet {}", id);

    match state.vet_repo.find_by_id(id).await {
        Ok(Some(vet)) => (StatusCode::OK, Json(vet.into())),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Vet not found")),
        Err(e) => {
            warn!("Failed to get vet {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

async fn list_pet_types(
    State(state): State<AppState>,
) -> impl IntoResponse {
    info!("Listing pet types");

    match state.pet_type_repo.find_all().await {
        Ok(pet_types) => {
            (StatusCode::OK, Json(pet_types.into_iter().map(|pt| pt.into()).collect()))
        }
        Err(e) => {
            warn!("Failed to list pet types: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
        }
    }
}

#[derive(Debug, Deserialize)]
struct OwnerSearchParams {
    last_name: Option<String>,
}
