-- Generates a table for molecules with some properties at normal conditions
CREATE TABLE molecules (
  molecule_id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  formula  TEXT NOT NULL,
  density FLOAT NULL,
  molar_mass FLOAT NULL,
  acentric_factor FLOAT NULL,
  melting_point FLOAT NULL,
  boiling_point FLOAT NULL,
  critical_temperature FLOAT NULL,
  critical_pressure FLOAT NULL
)
