//! Repository Layer
//!
//! This module defines the repository interfaces that abstract
//! data access operations from business logic.

use async_trait::async_trait;
use chrono::{DateTime, Local};
use std::collections::HashSet;

use super::entities::{
    EntityValidationError, Owner, Pet, PetType, Specialty, Visit, Vet,
};

/// Result type for repository operations
pub type RepositoryResult<T> = Result<T, RepositoryError>;

/// Repository error types
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(#[from] EntityValidationError),
    #[error("Entity not found: {0}")]
    NotFound(String),
    #[error("Duplicate entity: {0}")]
    Duplicate(String),
    #[error("Transaction error: {0}")]
    Transaction(String),
}

impl RepositoryError {
    pub fn is_not_found(&self) -> bool {
        matches!(self, RepositoryError::NotFound(_))
    }
}

/// Owner repository contract
#[async_trait]
pub trait OwnerRepository: Send + Sync + fmt::Debug {
    /// Find owner by ID
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Owner>>;

    /// Find owner by last name
    async fn find_by_last_name(&self, last_name: &str) -> RepositoryResult<Vec<Owner>>;

    /// Find all owners
    async fn find_all(&self) -> RepositoryResult<Vec<Owner>>;

    /// Save a new owner or update existing
    async fn save(&self, owner: &Owner) -> RepositoryResult<Owner>;

    /// Update owner
    async fn update(&self, owner: &Owner) -> RepositoryResult<Owner>;

    /// Delete owner by ID
    async fn delete(&self, id: i32) -> RepositoryResult<()>;

    /// Check if owner exists
    async fn exists(&self, id: i32) -> RepositoryResult<bool>;
}

/// Pet type repository contract
#[async_trait]
pub trait PetTypeRepository: Send + Sync + fmt::Debug {
    /// Find all pet types
    async fn find_all(&self) -> RepositoryResult<Vec<PetType>>;

    /// Find pet type by ID
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<PetType>>;
}

/// Pet repository contract
#[async_trait]
pub trait PetRepository: Send + Sync + fmt::Debug {
    /// Find pet by ID
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Pet>>;

    /// Find all pets
    async fn find_all(&self) -> RepositoryResult<Vec<Pet>>;

    /// Find pets by owner ID
    async fn find_by_owner_id(&self, owner_id: i32) -> RepositoryResult<Vec<Pet>>;

    /// Find pets by type ID
    async fn find_by_type_id(&self, type_id: i32) -> RepositoryResult<Vec<Pet>>;

    /// Save a new pet or update existing
    async fn save(&self, pet: &Pet) -> RepositoryResult<Pet>;

    /// Update pet
    async fn update(&self, pet: &Pet) -> RepositoryResult<Pet>;

    /// Delete pet by ID
    async fn delete(&self, id: i32) -> RepositoryResult<()>;

    /// Check if pet exists
    async fn exists(&self, id: i32) -> RepositoryResult<bool>;
}

/// Visit repository contract
#[async_trait]
pub trait VisitRepository: Send + Sync + fmt::Debug {
    /// Find visit by ID
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Visit>>;

    /// Find all visits
    async fn find_all(&self) -> RepositoryResult<Vec<Visit>>;

    /// Find visits by pet ID
    async fn find_by_pet_id(&self, pet_id: i32) -> RepositoryResult<Vec<Visit>>;

    /// Find visits by date range
    async fn find_by_date_range(
        &self,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    ) -> RepositoryResult<Vec<Visit>>;

    /// Save a new visit or update existing
    async fn save(&self, visit: &Visit) -> RepositoryResult<Visit>;

    /// Update visit
    async fn update(&self, visit: &Visit) -> RepositoryResult<Visit>;

    /// Delete visit by ID
    async fn delete(&self, id: i32) -> RepositoryResult<()>;

    /// Check if visit exists
    async fn exists(&self, id: i32) -> RepositoryResult<bool>;
}

/// Vet repository contract
#[async_trait]
pub trait VetRepository: Send + Sync + fmt::Debug {
    /// Find vet by ID
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Vet>>;

    /// Find all vets
    async fn find_all(&self) -> RepositoryResult<Vec<Vet>>;

    /// Find vets by specialty ID
    async fn find_by_specialty_id(&self, specialty_id: i32) -> RepositoryResult<Vec<Vet>>;

    /// Save a new vet or update existing
    async fn save(&self, vet: &Vet) -> RepositoryResult<Vet>;

    /// Update vet
    async fn update(&self, vet: &Vet) -> RepositoryResult<Vet>;

    /// Delete vet by ID
    async fn delete(&self, id: i32) -> RepositoryResult<()>;

    /// Check if vet exists
    async fn exists(&self, id: i32) -> RepositoryResult<bool>;
}

/// Specialty repository contract
#[async_trait]
pub trait SpecialtyRepository: Send + Sync + fmt::Debug {
    /// Find specialty by ID
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Specialty>>;

    /// Find all specialties
    async fn find_all(&self) -> RepositoryResult<Vec<Specialty>>;

    /// Save a specialty
    async fn save(&self, specialty: &Specialty) -> RepositoryResult<Specialty>;
}

/// Unit of Work - manages transactions
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    /// Start a transaction
    async fn begin(&self) -> RepositoryResult<()>;

    /// Commit the transaction
    async fn commit(&self) -> RepositoryResult<()>;

    /// Rollback the transaction
    async fn rollback(&self) -> RepositoryResult<()>;

    /// Get owner repository
    fn owner_repository(&self) -> &dyn OwnerRepository;

    /// Get pet repository
    fn pet_repository(&self) -> &dyn PetRepository;

    /// Get visit repository
    fn visit_repository(&self) -> &dyn VisitRepository;

    /// Get vet repository
    fn vet_repository(&self) -> &dyn VetRepository;

    /// Get specialty repository
    fn specialty_repository(&self) -> &dyn SpecialtyRepository;
}
