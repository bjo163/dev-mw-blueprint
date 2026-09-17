# ADR-001 — Mizan Engine Implementation Language

**Status:** ACCEPTED  
**Date:** 2026-09-18  
**Repository:** `bjo163/dev-mw-blueprint`  
**Decision:** Rust for the canonical Mizan rule engine.

---

## 1. Decision

The canonical executable implementation of the MoonWitness Mizan ontology, routing validator, role-state resolver, and TH responsibility resolver SHALL be implemented in **Rust**.

Markdown and JSON remain the normative blueprint/contract formats. Rust consumes those contracts and produces deterministic analysis results.

TypeScript MAY be used later for API/UI adapters, but it MUST NOT become a second independent implementation of Mizan rules.

```text
MARKDOWN / JSON
      ↓
CANONICAL CONTRACT
      ↓
RUST MIZAN CORE
      ↓
API / CLI / WASM ADAPTERS
      ↓
WEB / ADMIN / OTHER CLIENTS
```

---

## 2. Why Rust

The Mizan core is primarily a deterministic rules-and-validation engine, not a generative application layer. Rust is selected because the implementation benefits from:

- explicit enums for structural levels, domains, activities, mission types, mandate sources, and authority dimensions;
- exhaustive `match` checking when ontology variants are added;
- strong separation between stable identity and event-active role;
- predictable deterministic execution;
- safe numeric handling for TH classes and future scoring components;
- easy unit/property testing;
- CLI/server/WASM reuse from a single core implementation;
- no runtime dependency on Python.

The architecture MUST avoid encoding theological or civil semantics as untyped strings inside business logic when a closed enum or validated identifier is appropriate.

---

## 3. Canonical Dimensions

The Rust core MUST preserve the current dimensional separation:

```text
LEVEL      = structural position/domain
DOMAIN     = relationship/context domain
ACTIVITY   = what is being done
ROLE       = event-active function
MISSION    = entrusted objective
MANDATE    = source/scope of authorized responsibility
AUTHORITY  = authority dimension(s)
TH         = event responsibility weight
```

Critical invariant:

```text
LEVEL != WEIGHT
WEIGHT != STATUS
MISSION != IDENTITY
AUTHORITY != DIVINE STATUS
CORRECTIVE != LEVEL
COUNSEL_AUTHORITY != ADMIN_AUTHORITY
```

`CORRECTIVE_GUIDANCE` is an activity that can occur across levels. It MUST NOT be represented as a structural level.

---

## 4. Initial Rust Workspace

Recommended workspace:

```text
crates/
  mizan-model/        # enums, IDs, domain types, event structures
  mizan-contract/     # JSON loading + schema/contract validation
  mizan-th/           # TH resolver
  mizan-routing/      # route and no-bypass validator
  mizan-engine/       # orchestration / Mizan event evaluation
  mizan-ledger/       # result/evidence/provenance structures
  mizan-cli/          # local deterministic CLI
```

Possible later adapters:

```text
crates/
  mizan-api/          # HTTP adapter if needed
  mizan-wasm/         # browser/client execution if needed
```

The domain logic MUST stay below adapters. HTTP, UI, persistence, and presentation MUST NOT own canonical Mizan rules.

---

## 5. Core Rust Types

The first implementation SHOULD model at least:

```text
StructuralLevel
RelationshipDomain
ActivityType
ActiveRole
MissionType
MandateSource
AuthorityDimension
ThClass
EventContext
EvidenceRef
Provenance
ReasonCode
RoutingDecision
MizanEvent
MizanResult
```

Representative shape:

```rust
pub enum StructuralLevel {
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
}

pub enum ActivityType {
    Balagh,
    Guidance,
    Nasihah,
    CorrectiveGuidance,
    Administration,
    Justice,
    LawEnforcement,
    Healing,
    Education,
    Care,
    GeneralCivil,
}

pub enum ThClass {
    SpecialMission33,
    Expert17,
    Functional5,
    HumanBaseline2_5,
    Passive1,
    Trace(f64),
    Unknown,
}
```

Exact naming may evolve, but the separation of dimensions MUST remain intact.

---

## 6. TH Resolver Rule

TH MUST NOT be calculated from level alone.

Forbidden:

```text
TH = f(level)
```

Required conceptual interface:

```text
resolve_th(
  active_role,
  activities,
  mission_types,
  mandate_sources,
  authority_dimensions,
  knowledge_scope,
  event_scope,
  emergency_state,
  causal_involvement,
  evidence
) -> TH + reason_codes
```

Every non-UNKNOWN TH result MUST contain reason codes and evidence/provenance references where applicable.

---

## 7. Family / Nasihah Rule

The engine MUST represent family/kinship separately from corrective activity.

Example:

```text
STRUCTURAL LEVEL = L2
RELATION DOMAIN  = FAMILY_KINSHIP
RELATION         = CHILD -> PARENT
ACTIVITY         = CORRECTIVE_GUIDANCE
AUTHORITY        = COUNSEL_AUTHORITY
ADMIN AUTHORITY  = FALSE / NOT IMPLIED
TH               = resolved from active mission/context
```

Therefore:

```text
CHILD -> PARENT COUNSEL     = potentially valid
JUNIOR -> SENIOR COUNSEL    = potentially valid
COUNSEL -> ADMIN TAKEOVER   = invalid unless independently mandated
```

---

## 8. Governance Routing Rule

The Rust validator MUST distinguish direct guidance from civil administrative execution.

Valid guidance lane:

```text
RASUL/NABI -> BALAGH/GUIDANCE -> HUMAN
```

Civil execution lane:

```text
GUIDANCE/PRINCIPLE
-> ULUL AMRI / GOVERNMENT
-> ADMINISTRATION
-> STATE INSTITUTION
-> OFFICER / PUBLIC SERVICE
-> HUMAN / CIVIL
```

A direct guidance relation MUST NOT be silently converted into administrative execution authority.

---

## 9. JSON Contract Policy

The current JSON contracts remain input/reference artifacts:

- `mizan-governance-th.v1.json`
- `mizan-governance-th.v1.1.json`
- associated fixtures

Rust SHOULD validate and deserialize canonical contract versions with `serde`.

Version mismatch MUST fail explicitly or enter a documented compatibility path. Unknown enum values MUST NOT be silently normalized into an existing authority or mission.

---

## 10. Testing Baseline

Before API/UI work, the Rust core MUST pass fixtures covering at least:

1. Rasul direct guidance to human.
2. Ulama Mission #2 active.
3. Police off duty.
4. Police on duty.
5. Doctor normal clinical role.
6. Doctor emergency mission.
7. Judge active case.
8. Lawyer expert-only role.
9. Lawyer active representation mandate.
10. General citizen.
11. Child-to-parent corrective guidance.
12. Junior expert-to-senior counsel without authority takeover.

Additional negative tests MUST verify authority-boundary violations.

---

## 11. Final Technology Boundary

```text
RUST
= canonical deterministic Mizan engine

JSON
= machine-readable ontology/contracts/fixtures

MARKDOWN
= research and human-readable specification

TYPESCRIPT
= optional API/UI/client integration layer
  NOT a second rule authority

PYTHON
= not required for the production Mizan implementation
```

This ADR is the current technical baseline for implementation.