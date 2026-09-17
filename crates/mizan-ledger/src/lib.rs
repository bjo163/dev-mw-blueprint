use mizan_model::{EvidenceRef, MizanEvent, MizanResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub record_id: String,
    pub occurred_at: String,
    pub event: MizanEvent,
    pub result: MizanResult,
    pub evidence: Vec<EvidenceRef>,
    pub provenance: Vec<String>,
}

impl LedgerEntry {
    pub fn new(
        record_id: impl Into<String>,
        occurred_at: impl Into<String>,
        event: MizanEvent,
        result: MizanResult,
        provenance: Vec<String>,
    ) -> Self {
        let evidence = event.evidence.clone();
        Self {
            record_id: record_id.into(),
            occurred_at: occurred_at.into(),
            event,
            result,
            evidence,
            provenance,
        }
    }
}
