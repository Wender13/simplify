CREATE TABLE IF NOT EXISTS conta (
    id_conta      INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario    INTEGER NOT NULL,
    nome          TEXT NOT NULL,
    tipo          TEXT NOT NULL,
    saldo_inicial DECIMAL(10,2) DEFAULT 0,
    moeda         TEXT DEFAULT 'BRL',
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario)
        ON DELETE RESTRICT ON UPDATE CASCADE
);
