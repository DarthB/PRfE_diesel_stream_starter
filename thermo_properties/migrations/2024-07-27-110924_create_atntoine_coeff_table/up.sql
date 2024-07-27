-- Your SQL goes here
CREATE TABLE antoine_coeff (
    id SERIAL PRIMARY KEY,
    mol_id INT NOT NULL REFERENCES molecules(molecule_id),
    low_temp FLOAT NOT NULL,
    max_temp FLOAT NOT NULL,
    a FLOAT NOT NULL,
    b FLOAT NOT NULL,
    c FLOAT NOT NULL
)
