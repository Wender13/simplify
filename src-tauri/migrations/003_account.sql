CREATE TABLE IF NOT EXISTS account (
    id_account      INTEGER PRIMARY KEY AUTOINCREMENT,
    id_user    INTEGER NOT NULL,
    name          TEXT NOT NULL,
    type          TEXT NOT NULL,
    openning_balance DECIMAL(10,2) DEFAULT 0,
    currency         TEXT DEFAULT 'BRL',
    FOREIGN KEY (id_user) REFERENCES user(id_user)
        ON DELETE RESTRICT ON UPDATE CASCADE
);
