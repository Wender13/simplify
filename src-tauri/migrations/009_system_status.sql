CREATE TABLE IF NOT EXISTS system_status (
  id_system_status INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  value TEXT NOT NULL,
  description TEXT NOT NULL
);
INSERT INTO system_status (name, value, description)
VALUES (
    'appConfigured',
    'false',
    'Verifies if the app is configured'
  );