CREATE OR REPLACE FUNCTION reject_mizan_ledger_mutation()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    RAISE EXCEPTION 'mizan_ledger is append-only; UPDATE/DELETE is prohibited';
END;
$$;

DROP TRIGGER IF EXISTS trg_mizan_ledger_append_only ON mizan_ledger;
CREATE TRIGGER trg_mizan_ledger_append_only
BEFORE UPDATE OR DELETE ON mizan_ledger
FOR EACH ROW
EXECUTE FUNCTION reject_mizan_ledger_mutation();
