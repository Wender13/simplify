CREATE TABLE IF NOT EXISTS transacao (
    id_transacao   INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario     INTEGER NOT NULL,
    id_categoria   INTEGER NOT NULL,
    id_conta       INTEGER NOT NULL,
    valor          DECIMAL(10,2) NOT NULL,
    tipo           TEXT NOT NULL CHECK (tipo IN ('RECEITA', 'DESPESA')),
    descricao      TEXT,
    data_transacao DATE NOT NULL,
    FOREIGN KEY (id_usuario)   REFERENCES usuario(id_usuario)
        ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (id_categoria) REFERENCES categoria(id_categoria)
        ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (id_conta)     REFERENCES conta(id_conta)
        ON DELETE RESTRICT ON UPDATE CASCADE
);
