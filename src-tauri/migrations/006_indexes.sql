CREATE INDEX IF NOT EXISTS idx_category_user ON category(id_user);
CREATE INDEX IF NOT EXISTS idx_account_user     ON account(id_user);
CREATE INDEX IF NOT EXISTS idx_transaction_user ON transaction(id_user);
CREATE INDEX IF NOT EXISTS idx_target_user      ON financial_target(id_user);

CREATE INDEX IF NOT EXISTS idx_transaction_category ON transaction(id_category);
CREATE INDEX IF NOT EXISTS idx_transaction_account     ON transaction(id_account);
CREATE INDEX IF NOT EXISTS idx_target_category      ON financial_target(id_category);

CREATE INDEX IF NOT EXISTS idx_transaction_user_date ON transaction(id_user, transaction_date);
CREATE INDEX IF NOT EXISTS idx_category_user_type ON category(id_user, type);
