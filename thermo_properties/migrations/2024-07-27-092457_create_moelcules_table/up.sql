-- Your SQL goes here
CREATE TABLE molecules (
  molecule_id SERIAL PRIMARY KEY,
  name TEXT,
  formula TEXT,
  density FLOAT NULL,
  molar_mass FLOAT NULL,
  acentric_factor FLOAT NULL,
  melting_point FLOAT NULL,
  boiling_point FLOAT NULL,
  critical_temperature FLOAT NULL,
  critical_pressure FLOAT NULL
)
