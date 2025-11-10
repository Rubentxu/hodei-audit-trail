-- =============================================================================
-- PET CLINIC DATABASE SCHEMA
-- =============================================================================

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================================================
-- PET TYPES TABLE
-- =============================================================================
CREATE TABLE types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(80) NOT NULL UNIQUE
);

-- =============================================================================
-- SPECIALTIES TABLE
-- =============================================================================
CREATE TABLE specialties (
    id SERIAL PRIMARY KEY,
    name VARCHAR(80) NOT NULL UNIQUE
);

-- =============================================================================
-- OWNERS TABLE
-- =============================================================================
CREATE TABLE owners (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    address VARCHAR(255),
    city VARCHAR(100),
    telephone VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- =============================================================================
-- PETS TABLE
-- =============================================================================
CREATE TABLE pets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    birth_date DATE,
    type_id INTEGER REFERENCES types(id),
    owner_id INTEGER REFERENCES owners(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- =============================================================================
-- VISITS TABLE
-- =============================================================================
CREATE TABLE visits (
    id SERIAL PRIMARY KEY,
    pet_id INTEGER REFERENCES pets(id) ON DELETE CASCADE,
    visit_date DATE NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- =============================================================================
-- VETS TABLE
-- =============================================================================
CREATE TABLE vets (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL
);

-- =============================================================================
-- VET SPECIALTIES (MANY-TO-MANY)
-- =============================================================================
CREATE TABLE vet_specialties (
    vet_id INTEGER REFERENCES vets(id) ON DELETE CASCADE,
    specialty_id INTEGER REFERENCES specialties(id) ON DELETE CASCADE,
    PRIMARY KEY (vet_id, specialty_id)
);

-- =============================================================================
-- INDEXES FOR PERFORMANCE
-- =============================================================================
CREATE INDEX idx_pets_owner_id ON pets(owner_id);
CREATE INDEX idx_pets_type_id ON pets(type_id);
CREATE INDEX idx_visits_pet_id ON visits(pet_id);
CREATE INDEX idx_visits_visit_date ON visits(visit_date);
CREATE INDEX idx_owners_last_name ON owners(last_name);
CREATE INDEX idx_vets_last_name ON vets(last_name);
CREATE INDEX idx_types_name ON types(name);
CREATE INDEX idx_specialties_name ON specialties(name);

-- =============================================================================
-- TRIGGERS FOR UPDATED_AT
-- =============================================================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_owners_updated_at
    BEFORE UPDATE ON owners
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_pets_updated_at
    BEFORE UPDATE ON pets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
