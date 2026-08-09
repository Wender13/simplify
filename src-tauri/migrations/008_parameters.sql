CREATE TABLE IF NOT EXISTS parameters (
  id_parameter INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  value TEXT NOT NULL,
  description TEXT NOT NULL
);
INSERT INTO parameters (name, value, description)
VALUES (
    'appLanguage',
    'pt_BR',
    'Determines app language'
  );