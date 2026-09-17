# DEV-MW-BLUEPRINT

MoonWitness conceptual, machine-readable, and executable Mizan blueprints.

## Core Documents

### 1. GOD-PROMISE MOONWITNESS

`GOD-PROMISE_MOONWITNESS.md`

Research foundation for the Monocentric `1 · 6 · 3` architecture, HIZAB, MIZAN, LEDGER, evidence classification, and accountability boundaries.

### 2. MIZAN Governance · Mission · TH

`MIZAN-GOVERNANCE-TH-BLUEPRINT.md`

Operational ontology for L0–L6, active role vs stable identity, Mission #1 / Mission #2 / civil mandate, governance routing, mandate sources, mission types, authority dimensions, dynamic TH responsibility weights, no-bypass rules, and Mizan event vectors.

### 3. Machine-Readable Contracts

- `mizan-governance-th.v1.json` — canonical base contract
- `mizan-governance-th.v1.1.json` — FAMILY/KINSHIP + NASIHAH/CORRECTIVE extension
- `mizan-analytical-factors.v1.2.json` — evidence/provenance + analytical factor/ARI extension
- `mizan-governance-th.fixtures.v1.json` — governance/TH baseline fixtures
- `mizan-governance-th.fixtures.v1.1.json` — family/nasihah extension fixtures
- `mizan-engine.fixtures.v1.json` + `v1.1` — executable structural pipeline fixtures
- `mizan-analytical.fixtures.v1.2.json` — analytical evidence/ARI fixtures

The Rust contract tests validate the full chain `v1.0 -> v1.1 -> v1.2`.

### 4. FAMILY / KINSHIP + NASIHAH

`MIZAN-FAMILY-NASIHAH-DOMAIN.md`

Adds `FAMILY_KINSHIP` as a relationship domain and `NASIHAH_CORRECTIVE_GUIDANCE` / `CorrectiveGuidance` as a cross-level activity. `COUNSEL_AUTHORITY` does not imply administrative or enforcement power.

`CORRECTIVE_GUIDANCE` is an **activity**, never a structural level.

### 5. Implementation Decision — Rust

`ADR-001-MIZAN-ENGINE-RUST.md`

Canonical executable rules live in Rust. JSON remains the contract/fixture format; Markdown remains the research/specification format. TypeScript may be an adapter/UI layer later, but it must not become a second independent rule authority.

## Executable Rust Workspace

```text
crates/
  mizan-model/        # typed raw input, evidence, factors, event/result model
  mizan-contract/     # v1.0/v1.1/v1.2 contract validation
  mizan-role/         # active role -> contextual structural level
  mizan-authority/    # role/context -> effective authority + boundary checks
  mizan-th/           # deterministic TH resolver
  mizan-routing/      # route + no-bypass + counsel-boundary validator
  mizan-evidence/     # evidence reliability, provenance, factor bindings
  mizan-factors/      # Intent/Impact/Scope/Context/Causal vector + ARI
  mizan-engine/       # full orchestration / Mizan calculation
  mizan-ledger/       # evidence/provenance result record
  mizan-cli/          # raw JSON MizanInput CLI
  mizan-api/          # Axum HTTP adapter + PostgreSQL append-only ledger
```

## Public Execution Flow

```text
MizanInput
   ↓
ROLE / LEVEL RESOLVER
   ↓
AUTHORITY RESOLVER
   ↓
TH RESOLVER
   ↓
ROUTING / NO-BYPASS VALIDATOR
   ↓
EVIDENCE + PROVENANCE ASSESSMENT
   ↓
INTENT / IMPACT / SCOPE / CONTEXT / CAUSAL VECTOR
   ↓
ARI-0.1.0 (when factors are complete)
   ↓
MizanCalculation
   ↓
HASHED APPEND-ONLY POSTGRES LEDGER
```

Callers do **not** supply a trusted structural level or trusted effective authority. The engine resolves them from role, mission, mandate, activity, and context.

## Fundamental Rules

```text
LEVEL ≠ WEIGHT
WEIGHT ≠ STATUS
MISSION ≠ IDENTITY
AUTHORITY ≠ DIVINE STATUS
SAME TH ≠ SAME POSITION
CORRECTIVE ≠ LEVEL
COUNSEL_AUTHORITY ≠ ADMIN_AUTHORITY
TH ≠ ARI
ARI ≠ MORAL WORTH
ARI ≠ SALVATION
ARI ≠ LEGAL VERDICT
ARI ≠ FINAL DIVINE JUDGMENT
UNKNOWN ≠ GUESSED VALUE
```

TH remains the event responsibility weight:

```text
TH33   SPECIAL MISSION / HIGH MANDATE
TH17   EXPERT / KNOWLEDGE RESPONSIBILITY
TH5    FUNCTIONAL / SOCIAL RESPONSIBILITY
TH2.5  GENERAL HUMAN BASELINE
TH1    PASSIVE / MINOR ROLE
TH0.x  TRACE / INDIRECT CONTRIBUTION
```

## Evidence / Provenance Engine

Each `EvidenceRef` may carry:

```text
id
source
kind
reliability
provenance[]
  source
  method
  reference
```

Evidence strength is reported independently from provenance completeness. Analytical factors may bind to evidence IDs. A missing binding reference is explicit and prevents an ARI from being marked evidence-supported.

Evidence reliability classes:

```text
Low
Medium
High
Verified
Unknown
```

## Analytical Factor Vector

Sprint 03 adds five event dimensions without replacing TH:

```text
INTENT
IMPACT
SCOPE
CONTEXT
CAUSAL CONTRIBUTION
```

Unknown dimensions are preserved as `Unknown`; they are not silently imputed.

### ARI — Analytical Responsibility Index

Current calibration: `ARI-0.1.0`.

```text
TH normalized          30%
Intent                 20%
Impact                 20%
Scope                  10%
Causal contribution    20%
× Context multiplier
= ARI 0..100
```

ARI is produced only when TH and all five analytical factors are known. The engine separately reports whether the resulting ARI is adequately supported by evidence bindings and reliability. Therefore a numerical ARI can be marked **provisional** when evidence support is insufficient.

ARI is a versioned analytical comparison value inside MoonWitness. It is not a statement of human worth, holiness, guilt, salvation, heaven/hell, or final divine judgment.

## HTTP API

The API is an adapter around the same Rust engine; it does not duplicate Mizan rules.

```text
GET  /health
GET  /openapi.yaml
POST /api/v1/analyze
POST /api/v1/evaluate
POST /api/v1/resolve-role
POST /api/v1/resolve-authority
POST /api/v1/resolve-th
POST /api/v1/validate-route
POST /api/v1/assess-evidence
POST /api/v1/score-factors
```

OpenAPI source: `openapi/mizan-api.v1.yaml`.

## CLI / Examples

Run all tests:

```bash
cargo test --workspace --all-targets
```

Structural example:

```bash
cargo run -p mizan-cli -- examples/family-corrective-guidance.json
```

Evidence-backed analytical example:

```bash
cargo run -p mizan-cli -- examples/analytical-doctor.json
```

The CLI returns a complete `MizanCalculation` and exits non-zero when structural/authority routing validation fails.

## PostgreSQL Ledger

When `DATABASE_URL` is present, the API automatically runs embedded SQLx migrations and persists `/analyze` and `/evaluate` calls with hashed input/output plus version metadata.

Sprint 03 additionally persists:

```text
evidence_strength
analytical_responsibility_index
ari_calibration_version
ari_evidence_sufficient
```

The ledger is protected by database triggers against `UPDATE` and `DELETE` and remains an audit record of engine output. It is not a divine judgment ledger.

## Docker / Local Stack

```bash
docker compose up --build
```

This starts:

```text
PostgreSQL :5432
Mizan API  :3000
```

## Automated Coverage

Structural coverage contains **30 role/boundary cases**. Sprint 03 adds **10 analytical fixtures** covering:

- evidence-supported ARI
- high-mandate responsibility vector
- constrained accidental context
- unknown factor blocking ARI
- missing evidence binding producing provisional ARI
- low-reliability evidence producing provisional ARI
- emergency professional context
- trace contribution vs separate causal contribution
- provenance completeness reporting
- backward compatibility for legacy inputs without analytical factors

API tests cover health, structural evaluation, authority boundaries, evidence assessment, factor scoring, PostgreSQL persistence, analytical metadata persistence, and append-only mutation rejection.

GitHub Actions starts PostgreSQL and runs:

```bash
cargo test --workspace --all-targets
```

on pushes to `main` and pull requests.

## Current Baseline

**Ontology:** v1.2 analytical baseline  
**Contract chain:** v1.0 + v1.1 + v1.2  
**Structural pipeline:** role + authority + TH + routing  
**Analytical pipeline:** evidence + provenance + intent + impact + scope + context + causal contribution  
**ARI:** `ARI-0.1.0`  
**Structural fixtures:** 30  
**Analytical fixtures:** 10  
**HTTP:** Axum API v1.2  
**Ledger:** PostgreSQL append-only + hashes + version + analytical metadata  
**OpenAPI:** `openapi/mizan-api.v1.yaml`  
**Canonical engine:** Rust  
**Branch:** `main`
