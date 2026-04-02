CREATE TABLE IF NOT EXISTS usuario (
    id_usuario   INTEGER PRIMARY KEY AUTOINCREMENT,
    nome         TEXT NOT NULL,
    email        TEXT NOT NULL UNIQUE,
    senha_hash   TEXT NOT NULL,
    criado_em    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
