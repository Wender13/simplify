CREATE TRIGGER IF NOT EXISTS tr_atualiza_meta_transacao
AFTER INSERT ON transacao
FOR EACH ROW
WHEN NEW.tipo = 'DESPESA'
BEGIN
    UPDATE meta_financeira
    SET valor_atual = valor_atual + NEW.valor
    WHERE id_usuario   = NEW.id_usuario
      AND id_categoria = NEW.id_categoria
      AND ativa        = 1;
END;
