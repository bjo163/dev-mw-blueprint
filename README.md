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
- `mizan-governance-th.fixtures.v1.json` — governance/TH baseline fixtures
- `mizan-governance-th.fixtures.v1.1.json` — family/nasihah extension fixtures
- `mizan-engine.fixtures.v1.json` — executable baseline pipeline fixtures
- `mizan-engine.fixtures.v1.1.json` — extended role/boundary fixtures

The Rust contract tests validate both the v1 base contract and the v1.1 extension contract.

### 4. FAMILY / KINSHIP + NASIHAH

`MIZAN-FAMILY-NASIHAH-DOMAIN.md`

Adds:

- `FAMILY_KINSHIP` as a relationship domain
- `NASIHAH_CORRECTIVE_GUIDANCE` / `CorrectiveGuidance` as a cross-level activity
- `COUNSEL_AUTHORITY` / `Counsel` without implicit administrative or enforcement power
- upward, downward, and lateral counsel routing

`CORRECTIVE_GUIDANCE` is an **activity**, never a structural level.

### 5. Implementation Decision — Rust

`ADR-001-MIZAN-ENGINE-RUST.md`

Canonical executable rules live in Rust. JSON remains the contract/fixture format; Markdown remains the research/specification format. TypeScript may be an adapter/UI layer later, but it must not become a second independent rule authority.

## Executable Rust Workspace

```text
crates/
  mizan-model/        # typed raw input, resolved event, result model
  mizan-contract/     # base + extension contract validation
  mizan-role/         # active role -> contextual structural level
  mizan-authority/    # role/context -> effective authority + boundary checks
  mizan-th/           # deterministic TH resolver
  mizan-routing/      # route + no-bypass + counsel-boundary validator
  mizan-engine/       # full orchestration / Mizan calculation
  mizan-ledger/       # evidence/provenance result record
  mizan-cli/          # raw JSON MizanInput CLI
  mizan-api/          # Axum HTTP adapter + PostgreSQL immutable ledger
```

The public execution flow is:

```text
MizanInput
   ↓
ROLE / LEVEL RESOLVER
   ↓
AUTHORITY RESOLVER
   ↓
RESOLVED MizanEvent
   ↓
TH RESOLVER
   ↓
ROUTING / NO-BYPASS VALIDATOR
   ↓
MizanCalculation
   ├─ resolved_event
   ├─ TH class
   ├─ numeric th_value
   ├─ authority_valid
   ├─ denied_authorities
   ├─ routing decision
   ├─ structurally_valid
   ├─ evidence_sufficient
   └─ final_divine_judgment_computed = false
```

Callers do **not** supply a trusted structural level or trusted effective authority. The engine resolves them from the event role, mission, mandate, activity, and context.

## Fundamental Rules

```text
LEVEL ≠ WEIGHT
WEIGHT ≠ STATUS
MISSION ≠ IDENTITY
AUTHORITY ≠ DIVINE STATUS
SAME TH ≠ SAME POSITION
CORRECTIVE ≠ LEVEL
COUNSEL_AUTHORITY ≠ ADMIN_AUTHORITY
```

TH is an event responsibility weight, not human worth, holiness, salvation status, or divine favor.

```text
TH33   SPECIAL MISSION / HIGH MANDATE
TH17   EXPERT / KNOWLEDGE RESPONSIBILITY
TH5    FUNCTIONAL / SOCIAL RESPONSIBILITY
TH2.5  GENERAL HUMAN BASELINE
TH1    PASSIVE / MINOR ROLE
TH0.x  TRACE / INDIRECT CONTRIBUTION
```

Examples:

```text
Police off-duty general event  -> resolved L6 / TH2.5
Police active lawful mandate   -> resolved L4 / TH33
Doctor normal practice         -> L5 / TH17
Doctor emergency               -> L5 / TH33
Human family corrective advice -> L6 / TH5 / Counsel only
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

Direct guidance is not the same thing as direct civil administrative execution. Family seniority is not absolute authority, and counsel does not transfer administrative or enforcement power.

## CLI

Run all tests:

```bash
cargo test --workspace --all-targets
```

Evaluate a raw input:

```bash
cargo run -p mizan-cli -- examples/family-corrective-guidance.json
```

The CLI returns a complete `MizanCalculation` and exits non-zero when structural/authority routing validation fails.

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
```

OpenAPI source: `openapi/mizan-api.v1.yaml`.

Run without persistence:

```bash
cargo run -p mizan-api
```

Run with PostgreSQL ledger:

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/mizan
cargo run -p mizan-api
```

When `DATABASE_URL` is present the API automatically runs embedded SQLx migrations and persists successful `/analyze` and `/evaluate` calls with:

```text
record_id
occurred_at
actor_id
input_json
result_json
input_hash
result_hash
ontology_version
contract_version
engine_version
```

This persistence is an audit ledger for the engine output; it does not represent final divine judgment.

## Docker / Local Stack

```bash
docker compose up --build
```

This starts:

```text
PostgreSQL :5432
Mizan API  :3000
```

Copy `.env.example` when running services directly outside Compose.

## Automated Coverage

The full engine fixture suite now contains **30 cases**, including:

- Rasul and Nabi guidance missions
- Ulama knowledge role and Mission #2
- Ulul Amri / government context
- Police off-duty and on-duty
- Prosecutor and Judge
- Military defense
- Advocate active mandate and expert-only context
- Doctor normal and emergency response
- Scientist, Engineer, Teacher
- Parent, Guardian, Caregiver
- general citizen, passive role, trace contribution
- family and cross-level corrective counsel
- rejected Human -> Admin escalation
- rejected Doctor -> Enforcement escalation
- rejected Police -> Medical escalation
- rejected Ulama -> Enforcement escalation

API tests cover health, raw-input evaluation, role resolution, authority-boundary rejection, and PostgreSQL persistence.

GitHub Actions at `.github/workflows/rust-ci.yml` starts PostgreSQL and runs:

```bash
cargo test --workspace --all-targets
```

on pushes to `main` and pull requests.

## Current Baseline

**Ontology:** v1.1 conceptual baseline  
**Governance/TH contract:** v1 base + v1.1 extension  
**Executable pipeline:** role + authority + TH + routing + 30 fixture cases  
**HTTP:** Axum API v1  
**Ledger:** PostgreSQL + input/result hashes + version metadata  
**OpenAPI:** `openapi/mizan-api.v1.yaml`  
**Local delivery:** Docker Compose  
**Canonical engine:** Rust  
**Branch:** `main`
