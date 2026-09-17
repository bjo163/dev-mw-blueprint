# DEV-MW-BLUEPRINT

MoonWitness conceptual and machine-readable blueprints.

## Core Documents

### 1. GOD-PROMISE MOONWITNESS

`GOD-PROMISE_MOONWITNESS.md`

Research foundation for the Monocentric `1 · 6 · 3` architecture, HIZAB, MIZAN, LEDGER, evidence classification, and accountability boundaries.

### 2. MIZAN Governance · Mission · TH

`MIZAN-GOVERNANCE-TH-BLUEPRINT.md`

Operational ontology for:

- L0–L6 structural domains
- active role vs stable identity
- Mission #1 / Mission #2 / civil mandate
- governance routing
- `MANDATE_SOURCE`
- `MISSION_TYPE`
- authority dimensions
- dynamic TH responsibility weights
- no-bypass rules
- Mizan event vector
- invariants

### 3. Machine-Readable Contract

`mizan-governance-th.v1.json`

Canonical v1 contract for future API, DB, validator, and test implementations.

### 4. FAMILY / KINSHIP + NASIHAH Extension

`MIZAN-FAMILY-NASIHAH-DOMAIN.md`

Adds two dimensions that were missing from the first governance baseline:

- `FAMILY_KINSHIP` — relationship domain
- `NASIHAH_CORRECTIVE_GUIDANCE` — cross-level activity domain
- `COUNSEL_AUTHORITY` — advice/correction authority that does not imply administrative or enforcement power
- family, guardianship, warning, reminder, counsel, and corrective-guidance event fields
- upward / downward / lateral counsel routing

Machine-readable extension:

`mizan-governance-th.v1.1.json`

Additional fixtures:

`mizan-governance-th.fixtures.v1.1.json`

### 5. Implementation Decision — Rust

`ADR-001-MIZAN-ENGINE-RUST.md`

The canonical executable Mizan core is implemented in **Rust**.

Technology boundary:

```text
RUST       = canonical deterministic Mizan engine
JSON       = machine-readable contracts / fixtures
MARKDOWN   = research + human-readable specification
TYPESCRIPT = optional API / UI / client adapters
             NOT a second independent rule authority
```

Recommended Rust workspace:

```text
crates/
  mizan-model/
  mizan-contract/
  mizan-th/
  mizan-routing/
  mizan-engine/
  mizan-ledger/
  mizan-cli/
```

The core implementation keeps these dimensions distinct:

```text
LEVEL    = structural position/domain
DOMAIN   = relationship/context
ACTIVITY = what is being done
ROLE     = event-active function
MISSION  = entrusted objective
MANDATE  = source/scope of responsibility
AUTHORITY = permitted authority dimension
TH       = event responsibility weight
```

`CORRECTIVE_GUIDANCE` is an **activity**, not a structural level.

---

## Fundamental Rule

```text
LEVEL ≠ WEIGHT
WEIGHT ≠ STATUS
MISSION ≠ IDENTITY
AUTHORITY ≠ DIVINE STATUS
SAME TH ≠ SAME POSITION
CORRECTIVE ≠ LEVEL
COUNSEL_AUTHORITY ≠ ADMIN_AUTHORITY
```

TH is an **event responsibility weight**, not human worth or divine status.

```text
TH33   SPECIAL MISSION / HIGH MANDATE
TH17   EXPERT / KNOWLEDGE RESPONSIBILITY
TH5    FUNCTIONAL / SOCIAL RESPONSIBILITY
TH2.5  GENERAL HUMAN BASELINE
TH1    PASSIVE / MINOR ROLE
TH0.x  TRACE / INDIRECT CONTRIBUTION
```

## Core Routing

```text
MESSAGE / GUIDANCE
ALLAH -> REVELATION -> RASUL/NABI -> BALAGH/GUIDANCE -> HUMAN

RELIGIOUS STEWARDSHIP
REVELATION/GUIDANCE -> RASUL/NABI -> ULAMA/RELIGIOUS SCHOLAR -> COMMUNITY

CIVIL EXECUTION
GUIDANCE/PRINCIPLE -> ULUL AMRI/GOVERNMENT -> ADMINISTRATION
-> STATE INSTITUTION -> PUBLIC SERVICE/OFFICER -> HUMAN/CIVIL

FAMILY GUIDANCE
KNOWLEDGE/CONCERN -> FAMILY RELATION -> COUNSEL/WARNING/CORRECTIVE GUIDANCE -> FAMILY MEMBER

CROSS-LEVEL COUNSEL
RELEVANT KNOWLEDGE/EVIDENCE -> COUNSEL -> HIGHER/LOWER/PEER ROLE
```

Direct guidance is not the same thing as direct civil administrative execution.

Family seniority is not absolute authority, and counsel does not transfer administrative power.

## Current Baseline

**Ontology:** v1.1 conceptual baseline  
**Governance/TH contract:** v1.1 extension over v1.0  
**Canonical engine:** Rust  
**Branch:** `main`
