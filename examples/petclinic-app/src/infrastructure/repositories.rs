//! SQLx Repositories
//!
//! This module contains the concrete implementations of repository
//! traits using SQLx and PostgreSQL.

use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDate};
use sqlx::{postgres::PgPool, Row};

use crate::domain::entities::{
    EntityValidationError, Owner, Pet, PetType, Specialty, Visit, Vet,
};
use crate::domain::repositories::{
    EntityValidationError as DomainValidationError, OwnerRepository, PetTypeRepository,
    PetRepository, VisitRepository, VetRepository, RepositoryError, RepositoryResult,
};
use std::collections::HashSet;

/// SQLx-based Owner Repository
#[derive(Debug)]
pub struct SqlxOwnerRepository {
    pool: PgPool,
}

impl SqlxOwnerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OwnerRepository for SqlxOwnerRepository {
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Owner>> {
        let row = sqlx::query!(
            r#"
            SELECT id, first_name, last_name, address, city, telephone, created_at, updated_at
            FROM owners
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Owner {
            id: Some(r.id),
            first_name: r.first_name,
            last_name: r.last_name,
            address: r.address,
            city: r.city,
            telephone: r.telephone,
            created_at: r.created_at.map(|t| t.with_timezone(&Local)),
            updated_at: r.updated_at.map(|t| t.with_timezone(&Local)),
        }))
    }

    async fn find_by_last_name(&self, last_name: &str) -> RepositoryResult<Vec<Owner>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, first_name, last_name, address, city, telephone, created_at, updated_at
            FROM owners
            WHERE last_name ILIKE $1
            ORDER BY last_name, first_name
            "#,
            format!("{}%", last_name)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Owner {
                id: Some(r.id),
                first_name: r.first_name,
                last_name: r.last_name,
                address: r.address,
                city: r.city,
                telephone: r.telephone,
                created_at: r.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: r.updated_at.map(|t| t.with_timezone(&Local)),
            })
            .collect())
    }

    async fn find_all(&self) -> RepositoryResult<Vec<Owner>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, first_name, last_name, address, city, telephone, created_at, updated_at
            FROM owners
            ORDER BY last_name, first_name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Owner {
                id: Some(r.id),
                first_name: r.first_name,
                last_name: r.last_name,
                address: r.address,
                city: r.city,
                telephone: r.telephone,
                created_at: r.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: r.updated_at.map(|t| t.with_timezone(&Local)),
            })
            .collect())
    }

    async fn save(&self, owner: &Owner) -> RepositoryResult<Owner> {
        owner.validate().map_err(DomainValidationError::from)?;

        if let Some(id) = owner.id {
            // Update existing
            let row = sqlx::query!(
                r#"
                UPDATE owners
                SET first_name = $1,
                    last_name = $2,
                    address = $3,
                    city = $4,
                    telephone = $5,
                    updated_at = NOW()
                WHERE id = $6
                RETURNING id, first_name, last_name, address, city, telephone, created_at, updated_at
                "#,
                owner.first_name,
                owner.last_name,
                owner.address,
                owner.city,
                owner.telephone,
                id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(Owner {
                id: Some(row.id),
                first_name: row.first_name,
                last_name: row.last_name,
                address: row.address,
                city: row.city,
                telephone: row.telephone,
                created_at: row.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: row.updated_at.map(|t| t.with_timezone(&Local)),
            })
        } else {
            // Insert new
            let row = sqlx::query!(
                r#"
                INSERT INTO owners (first_name, last_name, address, city, telephone, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
                RETURNING id, first_name, last_name, address, city, telephone, created_at, updated_at
                "#,
                owner.first_name,
                owner.last_name,
                owner.address,
                owner.city,
                owner.telephone
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(Owner {
                id: Some(row.id),
                first_name: row.first_name,
                last_name: row.last_name,
                address: row.address,
                city: row.city,
                telephone: row.telephone,
                created_at: row.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: row.updated_at.map(|t| t.with_timezone(&Local)),
            })
        }
    }

    async fn update(&self, owner: &Owner) -> RepositoryResult<Owner> {
        self.save(owner).await
    }

    async fn delete(&self, id: i32) -> RepositoryResult<()> {
        sqlx::query!("DELETE FROM owners WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn exists(&self, id: i32) -> RepositoryResult<bool> {
        let row = sqlx::query!("SELECT id FROM owners WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.is_some())
    }
}

/// SQLx-based Pet Repository
#[derive(Debug)]
pub struct SqlxPetRepository {
    pool: PgPool,
}

impl SqlxPetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PetRepository for SqlxPetRepository {
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Pet>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, birth_date, type_id, owner_id, created_at, updated_at
            FROM pets
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            // Fetch visits
            let visits = sqlx::query!(
                r#"
                SELECT id, pet_id, visit_date, description, created_at
                FROM visits
                WHERE pet_id = $1
                ORDER BY visit_date DESC
                "#,
                r.id
            )
            .fetch_all(&self.pool)
            .await?;

            let pet_visits = visits
                .into_iter()
                .map(|v| Visit {
                    id: Some(v.id),
                    pet_id: v.pet_id,
                    visit_date: v.visit_date,
                    description: v.description,
                    created_at: v.created_at.map(|t| t.with_timezone(&Local)),
                })
                .collect();

            Ok(Some(Pet {
                id: Some(r.id),
                name: r.name,
                birth_date: r.birth_date,
                type_id: r.type_id,
                owner_id: r.owner_id,
                visits: pet_visits,
                created_at: r.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: r.updated_at.map(|t| t.with_timezone(&Local)),
            }))
        } else {
            Ok(None)
        }
    }

    async fn find_all(&self) -> RepositoryResult<Vec<Pet>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, birth_date, type_id, owner_id, created_at, updated_at
            FROM pets
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut pets = Vec::new();
        for r in rows {
            // Fetch visits for each pet
            let visits = sqlx::query!(
                r#"
                SELECT id, pet_id, visit_date, description, created_at
                FROM visits
                WHERE pet_id = $1
                ORDER BY visit_date DESC
                "#,
                r.id
            )
            .fetch_all(&self.pool)
            .await?;

            let pet_visits = visits
                .into_iter()
                .map(|v| Visit {
                    id: Some(v.id),
                    pet_id: v.pet_id,
                    visit_date: v.visit_date,
                    description: v.description,
                    created_at: v.created_at.map(|t| t.with_timezone(&Local)),
                })
                .collect();

            pets.push(Pet {
                id: Some(r.id),
                name: r.name,
                birth_date: r.birth_date,
                type_id: r.type_id,
                owner_id: r.owner_id,
                visits: pet_visits,
                created_at: r.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: r.updated_at.map(|t| t.with_timezone(&Local)),
            });
        }

        Ok(pets)
    }

    async fn find_by_owner_id(&self, owner_id: i32) -> RepositoryResult<Vec<Pet>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, birth_date, type_id, owner_id, created_at, updated_at
            FROM pets
            WHERE owner_id = $1
            ORDER BY name
            "#,
            owner_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut pets = Vec::new();
        for r in rows {
            let visits = sqlx::query!(
                r#"
                SELECT id, pet_id, visit_date, description, created_at
                FROM visits
                WHERE pet_id = $1
                ORDER BY visit_date DESC
                "#,
                r.id
            )
            .fetch_all(&self.pool)
            .await?;

            let pet_visits = visits
                .into_iter()
                .map(|v| Visit {
                    id: Some(v.id),
                    pet_id: v.pet_id,
                    visit_date: v.visit_date,
                    description: v.description,
                    created_at: v.created_at.map(|t| t.with_timezone(&Local)),
                })
                .collect();

            pets.push(Pet {
                id: Some(r.id),
                name: r.name,
                birth_date: r.birth_date,
                type_id: r.type_id,
                owner_id: r.owner_id,
                visits: pet_visits,
                created_at: r.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: r.updated_at.map(|t| t.with_timezone(&Local)),
            });
        }

        Ok(pets)
    }

    async fn find_by_type_id(&self, type_id: i32) -> RepositoryResult<Vec<Pet>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, birth_date, type_id, owner_id, created_at, updated_at
            FROM pets
            WHERE type_id = $1
            ORDER BY name
            "#,
            type_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Pet {
                id: Some(r.id),
                name: r.name,
                birth_date: r.birth_date,
                type_id: r.type_id,
                owner_id: r.owner_id,
                visits: Vec::new(),
                created_at: r.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: r.updated_at.map(|t| t.with_timezone(&Local)),
            })
            .collect())
    }

    async fn save(&self, pet: &Pet) -> RepositoryResult<Pet> {
        pet.validate().map_err(DomainValidationError::from)?;

        if let Some(id) = pet.id {
            let row = sqlx::query!(
                r#"
                UPDATE pets
                SET name = $1,
                    birth_date = $2,
                    type_id = $3,
                    owner_id = $4,
                    updated_at = NOW()
                WHERE id = $5
                RETURNING id, name, birth_date, type_id, owner_id, created_at, updated_at
                "#,
                pet.name,
                pet.birth_date,
                pet.type_id,
                pet.owner_id,
                id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(Pet {
                id: Some(row.id),
                name: row.name,
                birth_date: row.birth_date,
                type_id: row.type_id,
                owner_id: row.owner_id,
                visits: pet.visits.clone(),
                created_at: row.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: row.updated_at.map(|t| t.with_timezone(&Local)),
            })
        } else {
            let row = sqlx::query!(
                r#"
                INSERT INTO pets (name, birth_date, type_id, owner_id, created_at, updated_at)
                VALUES ($1, $2, $3, $4, NOW(), NOW())
                RETURNING id, name, birth_date, type_id, owner_id, created_at, updated_at
                "#,
                pet.name,
                pet.birth_date,
                pet.type_id,
                pet.owner_id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(Pet {
                id: Some(row.id),
                name: row.name,
                birth_date: row.birth_date,
                type_id: row.type_id,
                owner_id: row.owner_id,
                visits: pet.visits.clone(),
                created_at: row.created_at.map(|t| t.with_timezone(&Local)),
                updated_at: row.updated_at.map(|t| t.with_timezone(&Local)),
            })
        }
    }

    async fn update(&self, pet: &Pet) -> RepositoryResult<Pet> {
        self.save(pet).await
    }

    async fn delete(&self, id: i32) -> RepositoryResult<()> {
        sqlx::query!("DELETE FROM pets WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn exists(&self, id: i32) -> RepositoryResult<bool> {
        let row = sqlx::query!("SELECT id FROM pets WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.is_some())
    }
}

/// SQLx-based Visit Repository
#[derive(Debug)]
pub struct SqlxVisitRepository {
    pool: PgPool,
}

impl SqlxVisitRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VisitRepository for SqlxVisitRepository {
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Visit>> {
        let row = sqlx::query!(
            r#"
            SELECT id, pet_id, visit_date, description, created_at
            FROM visits
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Visit {
            id: Some(r.id),
            pet_id: r.pet_id,
            visit_date: r.visit_date,
            description: r.description,
            created_at: r.created_at.map(|t| t.with_timezone(&Local)),
        }))
    }

    async fn find_all(&self) -> RepositoryResult<Vec<Visit>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, pet_id, visit_date, description, created_at
            FROM visits
            ORDER BY visit_date DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Visit {
                id: Some(r.id),
                pet_id: r.pet_id,
                visit_date: r.visit_date,
                description: r.description,
                created_at: r.created_at.map(|t| t.with_timezone(&Local)),
            })
            .collect())
    }

    async fn find_by_pet_id(&self, pet_id: i32) -> RepositoryResult<Vec<Visit>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, pet_id, visit_date, description, created_at
            FROM visits
            WHERE pet_id = $1
            ORDER BY visit_date DESC
            "#,
            pet_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Visit {
                id: Some(r.id),
                pet_id: r.pet_id,
                visit_date: r.visit_date,
                description: r.description,
                created_at: r.created_at.map(|t| t.with_timezone(&Local)),
            })
            .collect())
    }

    async fn find_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> RepositoryResult<Vec<Visit>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, pet_id, visit_date, description, created_at
            FROM visits
            WHERE visit_date BETWEEN $1 AND $2
            ORDER BY visit_date DESC
            "#,
            start_date,
            end_date
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Visit {
                id: Some(r.id),
                pet_id: r.pet_id,
                visit_date: r.visit_date,
                description: r.description,
                created_at: r.created_at.map(|t| t.with_timezone(&Local)),
            })
            .collect())
    }

    async fn save(&self, visit: &Visit) -> RepositoryResult<Visit> {
        visit.validate().map_err(DomainValidationError::from)?;

        if let Some(id) = visit.id {
            let row = sqlx::query!(
                r#"
                UPDATE visits
                SET pet_id = $1,
                    visit_date = $2,
                    description = $3
                WHERE id = $4
                RETURNING id, pet_id, visit_date, description, created_at
                "#,
                visit.pet_id,
                visit.visit_date,
                visit.description,
                id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(Visit {
                id: Some(row.id),
                pet_id: row.pet_id,
                visit_date: row.visit_date,
                description: row.description,
                created_at: row.created_at.map(|t| t.with_timezone(&Local)),
            })
        } else {
            let row = sqlx::query!(
                r#"
                INSERT INTO visits (pet_id, visit_date, description, created_at)
                VALUES ($1, $2, $3, NOW())
                RETURNING id, pet_id, visit_date, description, created_at
                "#,
                visit.pet_id,
                visit.visit_date,
                visit.description
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(Visit {
                id: Some(row.id),
                pet_id: row.pet_id,
                visit_date: row.visit_date,
                description: row.description,
                created_at: row.created_at.map(|t| t.with_timezone(&Local)),
            })
        }
    }

    async fn update(&self, visit: &Visit) -> RepositoryResult<Visit> {
        self.save(visit).await
    }

    async fn delete(&self, id: i32) -> RepositoryResult<()> {
        sqlx::query!("DELETE FROM visits WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn exists(&self, id: i32) -> RepositoryResult<bool> {
        let row = sqlx::query!("SELECT id FROM visits WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.is_some())
    }
}

/// SQLx-based Vet Repository
#[derive(Debug)]
pub struct SqlxVetRepository {
    pool: PgPool,
}

impl SqlxVetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VetRepository for SqlxVetRepository {
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<Vet>> {
        let row = sqlx::query!(
            r#"
            SELECT id, first_name, last_name
            FROM vets
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            let specialties = sqlx::query!(
                r#"
                SELECT s.id, s.name
                FROM specialties s
                JOIN vet_specialties vs ON s.id = vs.specialty_id
                WHERE vs.vet_id = $1
                ORDER BY s.name
                "#,
                r.id
            )
            .fetch_all(&self.pool)
            .await?;

            let specialty_set = specialties
                .into_iter()
                .map(|s| Specialty {
                    id: s.id,
                    name: s.name,
                })
                .collect::<HashSet<_>>();

            Ok(Some(Vet {
                id: Some(r.id),
                first_name: r.first_name,
                last_name: r.last_name,
                specialties: specialty_set,
            }))
        } else {
            Ok(None)
        }
    }

    async fn find_all(&self) -> RepositoryResult<Vec<Vet>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, first_name, last_name
            FROM vets
            ORDER BY last_name, first_name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut vets = Vec::new();
        for r in rows {
            let specialties = sqlx::query!(
                r#"
                SELECT s.id, s.name
                FROM specialties s
                JOIN vet_specialties vs ON s.id = vs.specialty_id
                WHERE vs.vet_id = $1
                ORDER BY s.name
                "#,
                r.id
            )
            .fetch_all(&self.pool)
            .await?;

            let specialty_set = specialties
                .into_iter()
                .map(|s| Specialty {
                    id: s.id,
                    name: s.name,
                })
                .collect::<HashSet<_>>();

            vets.push(Vet {
                id: Some(r.id),
                first_name: r.first_name,
                last_name: r.last_name,
                specialties: specialty_set,
            });
        }

        Ok(vets)
    }

    async fn find_by_specialty_id(&self, specialty_id: i32) -> RepositoryResult<Vec<Vet>> {
        let rows = sqlx::query!(
            r#"
            SELECT v.id, v.first_name, v.last_name
            FROM vets v
            JOIN vet_specialties vs ON v.id = vs.vet_id
            WHERE vs.specialty_id = $1
            ORDER BY v.last_name, v.first_name
            "#,
            specialty_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut vets = Vec::new();
        for r in rows {
            let specialties = sqlx::query!(
                r#"
                SELECT s.id, s.name
                FROM specialties s
                JOIN vet_specialties vs ON s.id = vs.specialty_id
                WHERE vs.vet_id = $1
                ORDER BY s.name
                "#,
                r.id
            )
            .fetch_all(&self.pool)
            .await?;

            let specialty_set = specialties
                .into_iter()
                .map(|s| Specialty {
                    id: s.id,
                    name: s.name,
                })
                .collect::<HashSet<_>>();

            vets.push(Vet {
                id: Some(r.id),
                first_name: r.first_name,
                last_name: r.last_name,
                specialties: specialty_set,
            });
        }

        Ok(vets)
    }

    async fn save(&self, vet: &Vet) -> RepositoryResult<Vet> {
        if let Some(id) = vet.id {
            sqlx::query!(
                r#"
                UPDATE vets
                SET first_name = $1,
                    last_name = $2
                WHERE id = $3
                "#,
                vet.first_name,
                vet.last_name,
                id
            )
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query!(
                r#"
                INSERT INTO vets (first_name, last_name)
                VALUES ($1, $2)
                "#,
                vet.first_name,
                vet.last_name
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(vet.clone())
    }

    async fn update(&self, vet: &Vet) -> RepositoryResult<Vet> {
        self.save(vet).await
    }

    async fn delete(&self, id: i32) -> RepositoryResult<()> {
        sqlx::query!("DELETE FROM vets WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn exists(&self, id: i32) -> RepositoryResult<bool> {
        let row = sqlx::query!("SELECT id FROM vets WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.is_some())
    }
}

/// SQLx-based PetType Repository
#[derive(Debug)]
pub struct SqlxPetTypeRepository {
    pool: PgPool,
}

impl SqlxPetTypeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PetTypeRepository for SqlxPetTypeRepository {
    async fn find_all(&self) -> RepositoryResult<Vec<PetType>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name
            FROM types
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| PetType {
                id: r.id,
                name: r.name,
            })
            .collect())
    }

    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<PetType>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name
            FROM types
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| PetType {
            id: r.id,
            name: r.name,
        }))
    }
}
