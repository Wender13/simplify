CREATE TABLE IF NOT EXISTS transaction (
    id_transaction   INTEGER PRIMARY KEY AUTOINCREMENT,
    id_user     INTEGER NOT NULL,
    id_category   INTEGER NOT NULL,
    id_account       INTEGER NOT NULL,
    value          DECIMAL(10,2) NOT NULL,
    type           TEXT NOT NULL CHECK (tipo IN ('INCOME', 'EXPENSE')),
    description      TEXT,
    transaction_date DATE NOT NULL,
    FOREIGN KEY (id_user)   REFERENCES user(id_user)
        ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (id_category) REFERENCES category(id_category)
        ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY (id_account)     REFERENCES conta(id_account)
        ON DELETE RESTRICT ON UPDATE CASCADE
);
