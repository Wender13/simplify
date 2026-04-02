CREATE TABLE IF NOT EXISTS categoria (
    id_categoria INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario   INTEGER NOT NULL,
    nome         TEXT NOT NULL,
    tipo         TEXT NOT NULL CHECK (tipo IN ('RECEITA', 'DESPESA')),
    cor          TEXT,
    icone        TEXT,
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario)
        ON DELETE CASCADE ON UPDATE CASCADE
);
