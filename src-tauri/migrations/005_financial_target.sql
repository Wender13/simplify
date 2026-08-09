CREATE TABLE IF NOT EXISTS financial_target (
    id_target INTEGER PRIMARY KEY AUTOINCREMENT,
    id_user INTEGER NOT NULL,
    id_category INTEGER,
    name TEXT NOT NULL,
    limit_value DECIMAL(10, 2) NOT NULL,
    current_value DECIMAL(10, 2) DEFAULT 0,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    active INTEGER DEFAULT 1,
    FOREIGN KEY (id_user) REFERENCES user(id_user) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (id_category) REFERENCES category(id_category) ON DELETE
    SET NULL ON UPDATE CASCADE
);