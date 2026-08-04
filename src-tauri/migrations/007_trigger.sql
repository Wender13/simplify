CREATE TRIGGER IF NOT EXISTS tr_update_financial_target
AFTER INSERT ON transaction
FOR EACH ROW
WHEN NEW.type = 'EXPENSE'
BEGIN
    UPDATE financial_target
    SET current_value = current_value + NEW.value
    WHERE id_user = NEW.id_user
      AND id_category = NEW.id_category
      AND active = 1;
END;
