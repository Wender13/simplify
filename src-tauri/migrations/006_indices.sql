CREATE INDEX IF NOT EXISTS idx_categoria_usuario ON categoria(id_usuario);
CREATE INDEX IF NOT EXISTS idx_conta_usuario     ON conta(id_usuario);
CREATE INDEX IF NOT EXISTS idx_transacao_usuario ON transacao(id_usuario);
CREATE INDEX IF NOT EXISTS idx_meta_usuario      ON meta_financeira(id_usuario);

CREATE INDEX IF NOT EXISTS idx_transacao_categoria ON transacao(id_categoria);
CREATE INDEX IF NOT EXISTS idx_transacao_conta     ON transacao(id_conta);
CREATE INDEX IF NOT EXISTS idx_meta_categoria      ON meta_financeira(id_categoria);

CREATE INDEX IF NOT EXISTS idx_transacao_usuario_data ON transacao(id_usuario, data_transacao);
CREATE INDEX IF NOT EXISTS idx_categoria_usuario_tipo ON categoria(id_usuario, tipo);
