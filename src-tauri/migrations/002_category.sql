CREATE TABLE IF NOT EXISTS category (
    id_category INTEGER PRIMARY KEY AUTOINCREMENT,
    id_user   INTEGER NOT NULL,
    name         TEXT NOT NULL,
    type         TEXT NOT NULL CHECK (type IN ('INCOME', 'EXPENSE')),
    color          TEXT,
    icon        TEXT,
    FOREIGN KEY (id_user) REFERENCES user(id_user)
        ON DELETE CASCADE ON UPDATE CASCADE
);
