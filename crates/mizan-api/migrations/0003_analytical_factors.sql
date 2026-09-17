ALTER TABLE mizan_ledger
    ADD COLUMN IF NOT EXISTS evidence_strength TEXT,
    ADD COLUMN IF NOT EXISTS analytical_responsibility_index DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS ari_calibration_version TEXT,
    ADD COLUMN IF NOT EXISTS ari_evidence_sufficient BOOLEAN NOT NULL DEFAULT FALSE;

CREATE INDEX IF NOT EXISTS idx_mizan_ledger_evidence_strength
    ON mizan_ledger (evidence_strength);

CREATE INDEX IF NOT EXISTS idx_mizan_ledger_ari
    ON mizan_ledger (analytical_responsibility_index)
    WHERE analytical_responsibility_index IS NOT NULL;
