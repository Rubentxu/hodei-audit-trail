//! Application Services
//!
//! This module implements the application services that use
//! domain services and repositories to fulfill use cases.

use async_trait::async_trait;
use chrono::NaiveDate;

use crate::domain::entities::{Owner, Pet, PetType, Specialty, Visit, Vet};
use crate::domain::repositories::{
    OwnerRepository, PetRepository, VisitRepository, VetRepository, RepositoryResult,
    RepositoryError,
};
use crate::domain::services::{
    OwnerService, PetService, VisitService, VetService, BusinessRuleValidator,
    DomainServiceResult,
};

/// Owner application service
#[derive(Debug)]
pub struct OwnerApplicationService<R> {
    owner_repo: R,
    pet_repo: R::PetRepository,
    visit_repo: R::VisitRepository,
}

#[async_trait]
impl<R> OwnerService for OwnerApplicationService<R>
where
    R: OwnerRepository + PetRepository + VisitRepository + Send + Sync,
{
    async fn find_with_pets(&self, id: i32) -> RepositoryResult<Option<Owner>> {
        let owner = self.owner_repo.find_by_id(id).await?;

        if let Some(ref owner) = owner {
            let pets = self.pet_repo.find_by_owner_id(owner.id.unwrap()).await?;
            // Note: In a real implementation, we would need to fetch visits for each pet
        }

        Ok(owner)
    }

    async fn search_by_last_name(&self, last_name: &str) -> RepositoryResult<Vec<Owner>> {
        self.owner_repo.find_by_last_name(last_name).await
    }

    async fn create(&self, mut owner: Owner) -> DomainServiceResult<Owner> {
        owner.validate().map_err(DomainServiceError::from)?;
        let saved = self.owner_repo.save(&owner).await?;
        Ok(saved)
    }

    async fn update(&self, owner: &Owner) -> DomainServiceResult<Owner> {
        owner.validate().map_err(DomainServiceError::from)?;
        let updated = self.owner_repo.update(owner).await?;
        Ok(updated)
    }

    async fn delete_with_pets(&self, id: i32) -> DomainServiceResult<()> {
        // Check if owner exists
        if let Some(owner) = self.owner_repo.find_by_id(id).await? {
            // Find all pets and delete them
            let pets = self.pet_repo.find_by_owner_id(owner.id.unwrap()).await?;

            for pet in pets {
                if let Some(pet_id) = pet.id {
                    let visits = self.visit_repo.find_by_pet_id(pet_id).await?;
                    for visit in visits {
                        if let Some(visit_id) = visit.id {
                            self.visit_repo.delete(visit_id).await?;
                        }
                    }
                    self.pet_repo.delete(pet_id).await?;
                }
            }

            self.owner_repo.delete(id).await?;
            Ok(())
        } else {
            Err(DomainServiceError::BusinessRule("Owner not found".to_string()))
        }
    }
}

/// Pet application service
#[derive(Debug)]
pub struct PetApplicationService<R> {
    pet_repo: R::PetRepository,
    visit_repo: R::VisitRepository,
    owner_repo: R::OwnerRepository,
}

#[async_trait]
impl<R> PetService for PetApplicationService<R>
where
    R: PetRepository + VisitRepository + OwnerRepository + Send + Sync,
{
    async fn find_with_details(&self, id: i32) -> RepositoryResult<Option<Pet>> {
        let pet = self.pet_repo.find_by_id(id).await?;

        if let Some(ref pet) = pet {
            let visits = self.visit_repo.find_by_pet_id(pet.id.unwrap()).await?;
            // In a real implementation, we would combine pet with visits
        }

        Ok(pet)
    }

    async fn create(&self, mut pet: Pet) -> DomainServiceResult<Pet> {
        pet.validate().map_err(DomainServiceError::from)?;

        // Validate that owner exists
        if let Some(owner_id) = self.owner_repo.exists(pet.owner_id).await? {
            if !owner_id {
                return Err(DomainServiceError::BusinessRule(
                    "Owner does not exist".to_string(),
                ));
            }
        }

        let saved = self.pet_repo.save(&pet).await?;
        Ok(saved)
    }

    async fn update(&self, pet: &Pet) -> DomainServiceResult<Pet> {
        pet.validate().map_err(DomainServiceError::from)?;
        let updated = self.pet_repo.update(pet).await?;
        Ok(updated)
    }

    async fn delete(&self, id: i32) -> DomainServiceResult<()> {
        if let Some(pet) = self.pet_repo.find_by_id(id).await? {
            BusinessRuleValidator::validate_pet_can_be_deleted(&pet)?;
            self.pet_repo.delete(id).await?;
            Ok(())
        } else {
            Err(DomainServiceError::BusinessRule("Pet not found".to_string()))
        }
    }

    async fn add_visit(&self, mut visit: Visit) -> DomainServiceResult<Visit> {
        visit.validate().map_err(DomainServiceError::from)?;

        // Validate that pet exists
        if let Some(pet) = self.pet_repo.find_by_id(visit.pet_id).await? {
            let _ = pet; // Use pet for validation if needed
            let saved = self.visit_repo.save(&visit).await?;
            Ok(saved)
        } else {
            Err(DomainServiceError::BusinessRule("Pet not found".to_string()))
        }
    }
}

/// Visit application service
#[derive(Debug)]
pub struct VisitApplicationService<R> {
    visit_repo: R::VisitRepository,
    pet_repo: R::PetRepository,
}

#[async_trait]
impl<R> VisitService for VisitApplicationService<R>
where
    R: VisitRepository + PetRepository + Send + Sync,
{
    async fn find_with_pet(&self, id: i32) -> RepositoryResult<Option<Visit>> {
        let visit = self.visit_repo.find_by_id(id).await?;
        // In a real implementation, we would join with pet data
        Ok(visit)
    }

    async fn create(&self, mut visit: Visit) -> DomainServiceResult<Visit> {
        visit.validate().map_err(DomainServiceError::from)?;
        BusinessRuleValidator::validate_visit_schedule(&visit)?;

        // Validate that pet exists
        if !self.pet_repo.exists(visit.pet_id).await? {
            return Err(DomainServiceError::BusinessRule(
                "Pet does not exist".to_string(),
            ));
        }

        let saved = self.visit_repo.save(&visit).await?;
        Ok(saved)
    }

    async fn update(&self, visit: &Visit) -> DomainServiceResult<Visit> {
        visit.validate().map_err(DomainServiceError::from)?;
        let updated = self.visit_repo.update(visit).await?;
        Ok(updated)
    }

    async fn find_by_pet(&self, pet_id: i32) -> RepositoryResult<Vec<Visit>> {
        self.visit_repo.find_by_pet_id(pet_id).await
    }

    async fn find_by_date_range(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> RepositoryResult<Vec<Visit>> {
        self.visit_repo.find_by_date_range(start, end).await
    }
}

/// Vet application service
#[derive(Debug)]
pub struct VetApplicationService<R> {
    vet_repo: R::VetRepository,
    specialty_repo: R::SpecialtyRepository,
}

#[async_trait]
impl<R> VetService for VetApplicationService<R>
where
    R: VetRepository + SpecialtyRepository + Send + Sync,
{
    async fn find_with_specialties(&self, id: i32) -> RepositoryResult<Option<Vet>> {
        let vet = self.vet_repo.find_by_id(id).await?;
        // In a real implementation, we would join with specialties
        Ok(vet)
    }

    async fn find_all_with_specialties(&self) -> RepositoryResult<Vec<Vet>> {
        self.vet_repo.find_all().await
    }

    async fn create(&self, mut vet: Vet) -> DomainServiceResult<Vet> {
        let saved = self.vet_repo.save(&vet).await?;
        Ok(saved)
    }

    async fn update(&self, vet: &Vet) -> DomainServiceResult<Vet> {
        let updated = self.vet_repo.update(vet).await?;
        Ok(updated)
    }

    async fn add_specialty(&self, vet_id: i32, specialty_id: i32) -> DomainServiceResult<Vet> {
        if let Some(vet) = self.vet_repo.find_by_id(vet_id).await? {
            // In a real implementation, we would add the specialty to the vet
            let _ = specialty_id;
            Ok(vet)
        } else {
            Err(DomainServiceError::BusinessRule("Vet not found".to_string()))
        }
    }

    async fn remove_specialty(
        &self,
        vet_id: i32,
        specialty_id: i32,
    ) -> DomainServiceResult<Vet> {
        if let Some(vet) = self.vet_repo.find_by_id(vet_id).await? {
            // In a real implementation, we would remove the specialty from the vet
            let _ = specialty_id;
            Ok(vet)
        } else {
            Err(DomainServiceError::BusinessRule("Vet not found".to_string()))
        }
    }
}
