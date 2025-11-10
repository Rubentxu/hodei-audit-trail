//! Domain Services
//!
//! This module contains domain services that implement business logic
//! that doesn't naturally fit within a single entity.

use async_trait::async_trait;
use chrono::NaiveDate;
use std::collections::HashSet;

use super::entities::{Owner, Pet, PetType, Specialty, Visit, Vet};
use super::repositories::{RepositoryError, RepositoryResult};

/// Result type for domain services
pub type DomainServiceResult<T> = Result<T, DomainServiceError>;

/// Domain service errors
#[derive(Debug, thiserror::Error)]
pub enum DomainServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Business rule violation: {0}")]
    BusinessRule(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Concurrency error: {0}")]
    Concurrency(String),
}

impl From<RepositoryError> for DomainServiceError {
    fn from(err: RepositoryError) -> Self {
        DomainServiceError::Repository(err)
    }
}

/// Domain service for owner operations
#[async_trait]
pub trait OwnerService: Send + Sync {
    /// Find owner with pets
    async fn find_with_pets(&self, id: i32) -> RepositoryResult<Option<Owner>>;

    /// Search owners by last name
    async fn search_by_last_name(&self, last_name: &str) -> RepositoryResult<Vec<Owner>>;

    /// Create owner with validation
    async fn create(&self, owner: Owner) -> DomainServiceResult<Owner>;

    /// Update owner with business rules
    async fn update(&self, owner: &Owner) -> DomainServiceResult<Owner>;

    /// Delete owner and all associated pets
    async fn delete_with_pets(&self, id: i32) -> DomainServiceResult<()>;
}

/// Domain service for pet operations
#[async_trait]
pub trait PetService: Send + Sync {
    /// Find pet with owner and visits
    async fn find_with_details(&self, id: i32) -> RepositoryResult<Option<Pet>>;

    /// Create new pet
    async fn create(&self, pet: Pet) -> DomainServiceResult<Pet>;

    /// Update pet with validation
    async fn update(&self, pet: &Pet) -> DomainServiceResult<Pet>;

    /// Delete pet
    async fn delete(&self, id: i32) -> DomainServiceResult<()>;

    /// Add visit to pet
    async fn add_visit(&self, visit: Visit) -> DomainServiceResult<Visit>;
}

/// Domain service for visit operations
#[async_trait]
pub trait VisitService: Send + Sync {
    /// Find visit with pet details
    async fn find_with_pet(&self, id: i32) -> RepositoryResult<Option<Visit>>;

    /// Create new visit
    async fn create(&self, visit: Visit) -> DomainServiceResult<Visit>;

    /// Update visit
    async fn update(&self, visit: &Visit) -> DomainServiceResult<Visit>;

    /// Find visits by pet
    async fn find_by_pet(&self, pet_id: i32) -> RepositoryResult<Vec<Visit>>;

    /// Find visits by date range
    async fn find_by_date_range(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> RepositoryResult<Vec<Visit>>;
}

/// Domain service for vet operations
#[async_trait]
pub trait VetService: Send + Sync {
    /// Find vet with specialties
    async fn find_with_specialties(&self, id: i32) -> RepositoryResult<Option<Vet>>;

    /// Find all vets with specialties
    async fn find_all_with_specialties(&self) -> RepositoryResult<Vec<Vet>>;

    /// Create new vet
    async fn create(&self, vet: Vet) -> DomainServiceResult<Vet>;

    /// Update vet
    async fn update(&self, vet: &Vet) -> DomainServiceResult<Vet>;

    /// Add specialty to vet
    async fn add_specialty(&self, vet_id: i32, specialty_id: i32) -> DomainServiceResult<Vet>;

    /// Remove specialty from vet
    async fn remove_specialty(
        &self,
        vet_id: i32,
        specialty_id: i32,
    ) -> DomainServiceResult<Vet>;
}

/// Business rule validator
pub struct BusinessRuleValidator;

impl BusinessRuleValidator {
    /// Validate that owner can own a pet
    pub fn validate_owner_can_have_pet(_owner: &Owner, _pet: &Pet) -> DomainServiceResult<()> {
        // Business rules:
        // - Owner should not have more than 10 pets
        // - Pet name should be unique per owner
        // TODO: Add actual validation logic
        Ok(())
    }

    /// Validate visit schedule
    pub fn validate_visit_schedule(visit: &Visit) -> DomainServiceResult<()> {
        // Business rules:
        // - Visit date should not be in the future
        // - Description should be meaningful (not empty)
        // - Pet should exist
        Ok(())
    }

    /// Validate pet can be deleted
    pub fn validate_pet_can_be_deleted(pet: &Pet) -> DomainServiceResult<()> {
        // Business rules:
        // - Pet with active visits cannot be deleted
        // - Pet should be older than 30 days to delete
        if !pet.visits.is_empty() {
            return Err(DomainServiceError::BusinessRule(
                "Pet with visits cannot be deleted".to_string(),
            ));
        }
        Ok(())
    }
}
