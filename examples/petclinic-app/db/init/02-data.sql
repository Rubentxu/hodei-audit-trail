-- =============================================================================
-- PET CLINIC SAMPLE DATA
-- =============================================================================

-- -----------------------------------------------------------------------------
-- INSERT PET TYPES
-- -----------------------------------------------------------------------------
INSERT INTO types (name) VALUES
    ('Dog'),
    ('Cat'),
    ('Bird'),
    ('Rabbit'),
    ('Hamster'),
    ('Reptile'),
    ('Other');

-- -----------------------------------------------------------------------------
-- INSERT SPECIALTIES
-- -----------------------------------------------------------------------------
INSERT INTO specialties (name) VALUES
    ('Radiology'),
    ('Surgery'),
    ('Dentistry'),
    ('Internal Medicine'),
    ('Emergency and Critical Care'),
    ('Dermatology'),
    ('Oncology'),
    ('Cardiology'),
    ('Neurology');

-- -----------------------------------------------------------------------------
-- INSERT VETS
-- -----------------------------------------------------------------------------
INSERT INTO vets (first_name, last_name) VALUES
    ('James', 'Carter'),
    ('Helen', 'Leary'),
    ('Linda', 'Douglas'),
    ('Rafael', 'Ortega'),
    ('Henry', 'Stevens'),
    ('Sharon', 'Jenkins'),
    ('Emma', 'Wilson'),
    ('Michael', 'Brown'),
    ('Sarah', 'Davis');

-- -----------------------------------------------------------------------------
-- ASSIGN SPECIALTIES TO VETS
-- -----------------------------------------------------------------------------
-- James Carter - Radiology
INSERT INTO vet_specialties (vet_id, specialty_id) VALUES
    ((SELECT id FROM vets WHERE first_name = 'James' AND last_name = 'Carter'),
     (SELECT id FROM specialties WHERE name = 'Radiology'));

-- Helen Leary - Surgery
INSERT INTO vet_specialties (vet_id, specialty_id) VALUES
    ((SELECT id FROM vets WHERE first_name = 'Helen' AND last_name = 'Leary'),
     (SELECT id FROM specialties WHERE name = 'Surgery'));

-- Linda Douglas - Surgery and Emergency and Critical Care
INSERT INTO vet_specialties (vet_id, specialty_id) VALUES
    ((SELECT id FROM vets WHERE first_name = 'Linda' AND last_name = 'Douglas'),
     (SELECT id FROM specialties WHERE name = 'Surgery')),
    ((SELECT id FROM vets WHERE first_name = 'Linda' AND last_name = 'Douglas'),
     (SELECT id FROM specialties WHERE name = 'Emergency and Critical Care'));

-- Rafael Ortega - Surgery and Dentistry
INSERT INTO vet_specialties (vet_id, specialty_id) VALUES
    ((SELECT id FROM vets WHERE first_name = 'Rafael' AND last_name = 'Ortega'),
     (SELECT id FROM specialties WHERE name = 'Surgery')),
    ((SELECT id FROM vets WHERE first_name = 'Rafael' AND last_name = 'Ortega'),
     (SELECT id FROM specialties WHERE name = 'Dentistry'));

-- Henry Stevens - Radiology
INSERT INTO vet_specialties (vet_id, specialty_id) VALUES
    ((SELECT id FROM vets WHERE first_name = 'Henry' AND last_name = 'Stevens'),
     (SELECT id FROM specialties WHERE name = 'Radiology'));

-- Sharon Jenkins - Internal Medicine and Emergency and Critical Care
INSERT INTO vet_specialties (vet_id, specialty_id) VALUES
    ((SELECT id FROM vets WHERE first_name = 'Sharon' AND last_name = 'Jenkins'),
     (SELECT id FROM specialties WHERE name = 'Internal Medicine')),
    ((SELECT id FROM vets WHERE first_name = 'Sharon' AND last_name = 'Jenkins'),
     (SELECT id FROM specialties WHERE name = 'Emergency and Critical Care'));

-- Emma Wilson - Dermatology
INSERT INTO vet_specialties (vet_id, specialty_id) VALUES
    ((SELECT id FROM vets WHERE first_name = 'Emma' AND last_name = 'Wilson'),
     (SELECT id FROM specialties WHERE name = 'Dermatology'));

-- Michael Brown - Cardiology
INSERT INTO vet_specialties (vet_id, specialty_id) VALUES
    ((SELECT id FROM vets WHERE first_name = 'Michael' AND last_name = 'Brown'),
     (SELECT id FROM specialties WHERE name = 'Cardiology'));

-- Sarah Davis - Oncology
INSERT INTO vet_specialties (vet_id, specialty_id) VALUES
    ((SELECT id FROM vets WHERE first_name = 'Sarah' AND last_name = 'Davis'),
     (SELECT id FROM specialties WHERE name = 'Oncology'));

-- -----------------------------------------------------------------------------
-- INSERT SAMPLE OWNERS
-- -----------------------------------------------------------------------------
INSERT INTO owners (first_name, last_name, address, city, telephone) VALUES
    ('George', 'Franklin', '110 W. Liberty St.', 'Madison', '6085551023'),
    ('Betty', 'Davis', '638 Cardinal Ave.', 'Sun Prairie', '6085551745'),
    ('Eduardo', 'Rodriquez', '2693 Commerce St.', 'McFarland', '6085558763'),
    ('Harold', 'Davis', '563 Friendly St.', 'Windsor', '6085557334'),
    ('Peter', 'McTavish', '2387 S. Fair Way', 'Madison', '6085559675'),
    ('Jean', 'Coleman', '105 N. Lake St.', 'Monona', '6085557878'),
    ('Jeff', 'Black', '1450 Oak St.', 'Madison', '6085559834'),
    ('Maria', 'Escalante', '823 Maple St.', 'Waukesha', '6085558877'),
    ('David', 'Schroeder', '1025 Grand Ave.', 'Madison', '6085553487'),
    ('Carlos', 'Montgomery', '77 N. Chicago St.', 'Janesville', '6085557654'),
    ('Karen', 'White', '835 Highland Ave.', 'Madison', '6085559001'),
    ('Carol', 'Smith', '923 W. 2nd St.', 'Madison', '6085551200'),
    ('Tony', 'Shakespeare', '448 W. Broadway', 'Stoughton', '6085553456'),
    ('Anne', 'Taylor', '210 Queen St.', 'Madison', '6085552345'),
    ('Grace', 'Dawson', '110 N. Lake St.', 'Monona', '6085553333');

-- -----------------------------------------------------------------------------
-- INSERT SAMPLE PETS
-- -----------------------------------------------------------------------------
INSERT INTO pets (name, birth_date, type_id, owner_id) VALUES
    -- George Franklin's pets
    ('Leo', '2010-09-07', 1, 1),  -- Dog
    ('Basil', '2012-08-06', 2, 1), -- Cat

    -- Betty Davis' pets
    ('Rosy', '2011-04-17', 1, 2),  -- Dog
    ('Jewel', '2010-03-07', 1, 2), -- Dog
    ('Iggy', '2010-11-30', 3, 2),  -- Bird

    -- Eduardo Rodriquez's pets
    ('George', '2011-01-20', 1, 3),  -- Dog

    -- Harold Davis' pets
    ('Samantha', '2012-09-04', 2, 4), -- Cat
    ('Max', '2012-01-20', 1, 4),      -- Dog
    ('Lucky', '2011-08-06', 1, 4),    -- Dog

    -- Peter McTavish's pets
    ('Mulligan', '2007-02-24', 1, 5),  -- Dog
    ('Freddy', '2010-03-09', 2, 5),    -- Cat

    -- Jean Coleman's pets
    ('Lucky', '2010-06-24', 1, 6),     -- Dog
    ('Sly', '2012-01-13', 2, 6),       -- Cat

    -- Jeff Black's pets
    ('Molly', '2013-05-13', 1, 7),     -- Dog
    ('Gizmo', '2010-04-16', 1, 7),     -- Dog

    -- Maria Escalante's pets
    ('Nibbles', '2011-02-10', 4, 8),   -- Rabbit
    ('Nibbles Jr', '2012-05-05', 4, 8), -- Rabbit

    -- David Schroeder's pets
    ('Sparky', '2009-06-17', 1, 9),    -- Dog
    ('Tex', '2011-12-11', 1, 9),       -- Dog

    -- Carlos Montgomery's pets
    ('Freddy', '2010-03-12', 1, 10),   -- Dog
    ('Max II', '2011-01-20', 1, 10),   -- Dog

    -- Karen White's pets
    ('Jewel', '2012-07-31', 2, 11),    -- Cat
    ('Rocky', '2010-05-15', 1, 11),    -- Dog

    -- Carol Smith's pets
    ('Bailey', '2011-04-20', 1, 12),   -- Dog
    ('Coco', '2013-01-08', 2, 12),     -- Cat

    -- Tony Shakespeare's pets
    ('Birdie', '2010-08-19', 3, 13),   -- Bird
    ('Roxy', '2009-04-16', 1, 13),     -- Dog

    -- Anne Taylor's pets
    ('Whiskers', '2010-09-07', 2, 14), -- Cat
    ('Daisy', '2012-02-15', 1, 14),    -- Dog

    -- Grace Dawson's pets
    ('Scout', '2011-04-25', 1, 15),    -- Dog
    ('Shadow', '2008-02-20', 1, 15);   -- Dog

-- -----------------------------------------------------------------------------
-- INSERT SAMPLE VISITS
-- -----------------------------------------------------------------------------
INSERT INTO visits (pet_id, visit_date, description) VALUES
    -- Visits for Leo (George's dog)
    (1, '2013-01-01', 'rabies vaccination'),
    (1, '2013-01-12', 'rabies vaccination'),
    (1, '2013-03-15', 'annual checkup'),
    (1, '2013-05-20', 'tooth cleaning'),

    -- Visits for Basil (George's cat)
    (2, '2012-10-10', 'spayed'),
    (2, '2013-01-22', 'annual checkup'),
    (2, '2013-05-25', 'routine checkup'),

    -- Visits for Rosy (Betty's dog)
    (3, '2011-05-05', 'spayed'),
    (3, '2012-03-11', 'spayed'),
    (3, '2013-05-05', 'annual checkup'),

    -- Visits for Jewel (Betty's dog)
    (4, '2010-05-05', 'spayed'),
    (4, '2011-05-05', 'spayed'),
    (4, '2012-05-05', 'spayed'),
    (4, '2013-05-05', 'annual checkup'),

    -- Visits for Iggy (Betty's bird)
    (5, '2010-11-30', 'adopted'),
    (5, '2011-11-30', 'annual checkup'),
    (5, '2012-11-30', 'annual checkup'),
    (5, '2013-11-30', 'annual checkup'),

    -- Visits for George (Eduardo's dog)
    (6, '2011-01-20', 'adopted'),
    (6, '2011-02-15', 'neutered'),
    (6, '2011-05-20', 'first checkup'),
    (6, '2012-01-20', 'annual checkup'),
    (6, '2013-01-20', 'annual checkup'),

    -- Visits for Samantha (Harold's cat)
    (7, '2012-09-04', 'adopted'),
    (7, '2012-10-15', 'first checkup'),
    (7, '2012-12-01', 'spayed'),
    (7, '2013-09-04', 'annual checkup'),

    -- Visits for Max (Harold's dog)
    (8, '2012-01-20', 'adopted'),
    (8, '2012-02-20', 'neutered'),
    (8, '2013-01-20', 'annual checkup'),

    -- Visits for Lucky (Harold's dog)
    (9, '2011-08-06', 'adopted'),
    (9, '2012-08-06', 'annual checkup'),
    (9, '2013-08-06', 'annual checkup'),

    -- Visits for Mulligan (Peter's dog)
    (10, '2007-02-24', 'adopted'),
    (10, '2007-03-01', 'neutered'),
    (10, '2008-02-24', 'annual checkup'),
    (10, '2009-02-24', 'annual checkup'),
    (10, '2010-02-24', 'annual checkup'),
    (10, '2011-02-24', 'annual checkup'),
    (10, '2012-02-24', 'annual checkup'),
    (10, '2013-02-24', 'annual checkup'),

    -- Visits for Freddy (Peter's cat)
    (11, '2010-03-09', 'adopted'),
    (11, '2010-03-20', 'first checkup'),
    (11, '2011-03-09', 'annual checkup'),
    (11, '2012-03-09', 'annual checkup'),
    (11, '2013-03-09', 'annual checkup'),

    -- Visits for Lucky (Jean's dog)
    (12, '2010-06-24', 'adopted'),
    (12, '2010-07-15', 'neutered'),
    (12, '2011-06-24', 'annual checkup'),
    (12, '2012-06-24', 'annual checkup'),
    (12, '2013-06-24', 'annual checkup'),

    -- Visits for Sly (Jean's cat)
    (13, '2012-01-13', 'adopted'),
    (13, '2012-02-01', 'spayed'),
    (13, '2013-01-13', 'annual checkup'),

    -- Visits for Molly (Jeff's dog)
    (14, '2013-05-13', 'adopted'),
    (14, '2013-05-20', 'first checkup'),
    (14, '2013-06-10', 'neutered'),

    -- Visits for Gizmo (Jeff's dog)
    (15, '2010-04-16', 'adopted'),
    (15, '2010-05-01', 'neutered'),
    (15, '2011-04-16', 'annual checkup'),
    (15, '2012-04-16', 'annual checkup'),
    (15, '2013-04-16', 'annual checkup'),

    -- Visits for Nibbles (Maria's rabbit)
    (16, '2011-02-10', 'adopted'),
    (16, '2011-03-01', 'first checkup'),
    (16, '2012-02-10', 'annual checkup'),
    (16, '2013-02-10', 'annual checkup'),

    -- Visits for Nibbles Jr (Maria's rabbit)
    (17, '2012-05-05', 'adopted'),
    (17, '2012-05-20', 'first checkup'),
    (17, '2013-05-05', 'annual checkup'),

    -- Visits for Sparky (David's dog)
    (18, '2009-06-17', 'adopted'),
    (18, '2009-07-01', 'neutered'),
    (18, '2010-06-17', 'annual checkup'),
    (18, '2011-06-17', 'annual checkup'),
    (18, '2012-06-17', 'annual checkup'),
    (18, '2013-06-17', 'annual checkup'),

    -- Visits for Tex (David's dog)
    (19, '2011-12-11', 'adopted'),
    (19, '2012-01-15', 'neutered'),
    (19, '2012-12-11', 'annual checkup'),
    (19, '2013-12-11', 'routine checkup');

-- =============================================================================
-- VERIFY DATA
-- =============================================================================
SELECT 'Owners' as table_name, COUNT(*) as count FROM owners
UNION ALL
SELECT 'Pets', COUNT(*) FROM pets
UNION ALL
SELECT 'Visits', COUNT(*) FROM visits
UNION ALL
SELECT 'Vets', COUNT(*) FROM vets
UNION ALL
SELECT 'Specialties', COUNT(*) FROM specialties
UNION ALL
SELECT 'Types', COUNT(*) FROM types
UNION ALL
SELECT 'Vet Specialties', COUNT(*) FROM vet_specialties;
