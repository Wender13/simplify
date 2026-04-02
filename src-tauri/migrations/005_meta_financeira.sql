CREATE TABLE IF NOT EXISTS meta_financeira (
    id_meta      INTEGER PRIMARY KEY AUTOINCREMENT,
    id_usuario   INTEGER NOT NULL,
    id_categoria INTEGER,
    nome         TEXT NOT NULL,
    valor_limite DECIMAL(10,2) NOT NULL,
    valor_atual  DECIMAL(10,2) DEFAULT 0,
    data_inicio  DATE NOT NULL,
    data_fim     DATE NOT NULL,
    ativa        INTEGER DEFAULT 1,
    FOREIGN KEY (id_usuario)   REFERENCES usuario(id_usuario)
        ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (id_categoria) REFERENCES categoria(id_categoria)
        ON DELETE SET NULL ON UPDATE CASCADE
);
