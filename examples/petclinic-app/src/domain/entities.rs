//! Domain Entities
//!
//! Core business entities of the Pet Clinic domain.

use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use uuid::Uuid;

/// Owner of pets
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub telephone: Option<String>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Owner {
    /// Create a new owner
    pub fn new(
        first_name: String,
        last_name: String,
        address: Option<String>,
        city: Option<String>,
        telephone: Option<String>,
    ) -> Self {
        Self {
            id: None,
            first_name,
            last_name,
            address,
            city,
            telephone,
            created_at: None,
            updated_at: None,
        }
    }

    /// Get full name
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

/// Type of pet (Dog, Cat, Bird, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PetType {
    pub id: i32,
    pub name: String,
}

impl PetType {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

/// Pet entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pet {
    pub id: Option<i32>,
    pub name: String,
    pub birth_date: Option<NaiveDate>,
    pub type_id: i32,
    pub owner_id: i32,
    pub visits: Vec<Visit>, // Collection of visits
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Pet {
    /// Create a new pet
    pub fn new(name: String, birth_date: Option<NaiveDate>, type_id: i32, owner_id: i32) -> Self {
        Self {
            id: None,
            name,
            birth_date,
            type_id,
            owner_id,
            visits: Vec::new(),
            created_at: None,
            updated_at: None,
        }
    }

    /// Calculate age in years
    pub fn age(&self) -> Option<i32> {
        self.birth_date.map(|birth| {
            let now = Local::now().date_naive();
            let age = now.year() - birth.year();
            if now.month() < birth.month()
                || (now.month() == birth.month() && now.day() < birth.day())
            {
                age - 1
            } else {
                age
            }
        })
    }
}

/// Visit entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Visit {
    pub id: Option<i32>,
    pub pet_id: i32,
    pub visit_date: NaiveDate,
    pub description: String,
    pub created_at: Option<DateTime<Local>>,
}

impl Visit {
    /// Create a new visit
    pub fn new(pet_id: i32, visit_date: NaiveDate, description: String) -> Self {
        Self {
            id: None,
            pet_id,
            visit_date,
            description,
            created_at: None,
        }
    }
}

/// Veterinarian specialty (Radiology, Surgery, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Specialty {
    pub id: i32,
    pub name: String,
}

impl Specialty {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

/// Veterinarian
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vet {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub specialties: HashSet<Specialty>, // Many-to-Many
}

impl Vet {
    /// Create a new vet
    pub fn new(first_name: String, last_name: String) -> Self {
        Self {
            id: None,
            first_name,
            last_name,
            specialties: HashSet::new(),
        }
    }

    /// Get full name
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// Validation errors
#[derive(Debug, thiserror::Error)]
pub enum EntityValidationError {
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Date cannot be in the future")]
    FutureDate,
    #[error("Pet cannot be older than 50 years")]
    PetTooOld,
}

impl Owner {
    /// Validate owner data
    pub fn validate(&self) -> Result<(), EntityValidationError> {
        if self.first_name.trim().is_empty() {
            return Err(EntityValidationError::MissingField(
                "first_name".to_string(),
            ));
        }
        if self.last_name.trim().is_empty() {
            return Err(EntityValidationError::MissingField("last_name".to_string()));
        }
        Ok(())
    }
}

impl Pet {
    /// Validate pet data
    pub fn validate(&self) -> Result<(), EntityValidationError> {
        if self.name.trim().is_empty() {
            return Err(EntityValidationError::MissingField("name".to_string()));
        }
        if let Some(birth_date) = self.birth_date {
            let now = Local::now().date_naive();
            if birth_date > now {
                return Err(EntityValidationError::FutureDate);
            }
            if now.year() - birth_date.year() > 50 {
                return Err(EntityValidationError::PetTooOld);
            }
        }
        Ok(())
    }
}

impl Visit {
    /// Validate visit data
    pub fn validate(&self) -> Result<(), EntityValidationError> {
        if self.description.trim().is_empty() {
            return Err(EntityValidationError::MissingField(
                "description".to_string(),
            ));
        }
        let now = Local::now().date_naive();
        if self.visit_date > now {
            return Err(EntityValidationError::FutureDate);
        }
        Ok(())
    }
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

impl fmt::Display for Pet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Visit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Visit on {}: {}", self.visit_date, self.description)
    }
}
