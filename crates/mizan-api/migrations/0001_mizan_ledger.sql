CREATE TABLE IF NOT EXISTS mizan_ledger (
    record_id UUID PRIMARY KEY,
    occurred_at TIMESTAMPTZ NOT NULL,
    actor_id TEXT NOT NULL,
    input_json JSONB NOT NULL,
    result_json JSONB NOT NULL,
    input_hash TEXT NOT NULL,
    result_hash TEXT NOT NULL,
    ontology_version TEXT NOT NULL,
    contract_version TEXT NOT NULL,
    engine_version TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_mizan_ledger_actor_id
    ON mizan_ledger (actor_id);

CREATE INDEX IF NOT EXISTS idx_mizan_ledger_occurred_at
    ON mizan_ledger (occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_mizan_ledger_input_hash
    ON mizan_ledger (input_hash);
