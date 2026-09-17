# MIZAN — GOVERNANCE · MISSION · TH BLUEPRINT

**Status:** Baseline v1.0  
**Repository:** `bjo163/dev-mw-blueprint`  
**Target:** MoonWitness / Mizan  
**Purpose:** Operationalize role, mandate, mission, responsibility weight, routing, and accountability without collapsing theological identity, civil authority, and professional responsibility into one rank.

---

## 0. Core Rule

MoonWitness MUST keep these dimensions separate:

```text
LEVEL ≠ WEIGHT
WEIGHT ≠ STATUS
MISSION ≠ IDENTITY
AUTHORITY ≠ DIVINE STATUS
SAME TH ≠ SAME POSITION
SAME TH ≠ SAME SOURCE OF AUTHORITY
```

`LEVEL` answers: **what structural domain is active?**  
`ROLE` answers: **what function is the actor performing?**  
`MISSION` answers: **what entrusted objective is active?**  
`MANDATE_SOURCE` answers: **where does the authority for the role come from?**  
`TH` answers: **how heavy is the active responsibility/exposure for this event?**

TH is an analytical responsibility weight. It is **not** a measure of human worth, holiness, salvation, or divine favor.

---

# 1. L0–L6 Structural Ontology

The levels below are **typed domains**, not a single ladder of personal superiority.

## L0 — ALLAH

```text
TYPE          = ABSOLUTE_SOURCE
SCORABLE      = FALSE
TH            = NONE
```

- Allah stands alone at L0.
- L0 is not an object of Mizan scoring.
- `Tuhan`, `God`, `Rabb`, `Ilah`, `Dewa`, or other terms MUST NOT be silently normalized as aliases of `ALLAH`. They require explicit term/relation modeling.

---

## L1 — SPECIAL DIVINE RELATION / DESIGNATION / SUPPORT

Examples in the current framework:

```text
RELATION
- KHALIL
- TAKLIM

DESIGNATION
- KALIMATUHU
- RUHUN MINHU

SUPPORT / CARRIER
- RUH AL-QUDUS
```

```text
TYPE          = RELATION_DESIGNATION_SUPPORT
CIVIL_AUTH    = FALSE
GOVERNMENT    = FALSE
TH            = CONTEXTUAL / NOT FIXED
```

L1 MUST NOT be interpreted as “a person is administratively above L2.” It contains heterogeneous relation/designation/support concepts.

---

## L2 — DIVINE COMMISSION / MISSION #1

```text
ROLES
- RASUL
- NABI
```

Core functions:

```text
REVELATION
BALAGH
GUIDANCE
DIVINE_COMMISSION
```

Typical active weight:

```text
TH = 33 when mission is active
```

L2 does not automatically grant direct execution rights inside a later civil administrative system.

---

## L3 — RELIGIOUS STEWARDSHIP / MISSION #2

```text
ROLES
- ULAMA
- RELIGIOUS_SCHOLAR
- AHL_AL_DHIKR
- PEOPLE_OF_RELIGIOUS_KNOWLEDGE
- RELIGIOUS_TEACHER
- RELIGIOUS_STEWARD
```

Typical weights:

```text
KNOWLEDGE ROLE        = TH17
ACTIVE MISSION #2     = TH33
```

L3 is not police power, military power, judicial execution, or general state command merely because TH may reach 33.

---

## L4 — GOVERNANCE / SPECIAL CIVIL MANDATE

### Governance

```text
- ULUL_AMRI
- HEAD_OF_STATE
- GOVERNMENT_LEADER
- ADMINISTRATIVE_AUTHORITY
```

### Justice / Legal Execution

```text
- JUDGE
- PROSECUTOR
- COURT_AUTHORITY
- REGULATOR
```

### Protection / Enforcement

```text
- POLICE
- MILITARY
- LAW_ENFORCEMENT
- INTELLIGENCE
- SECURITY
- EMERGENCY_COMMAND
```

### Legal Representation

```text
- LAWYER / ADVOCATE when an active legal mandate is held
```

Typical active weight:

```text
ACTIVE SPECIAL CIVIL MANDATE = TH33
```

TH33 here means **high mission/mandate responsibility**, not prophetic identity.

---

## L5 — EXPERT / PROFESSIONAL / PUBLIC-SERVICE / SOCIAL RESPONSIBILITY

```text
- DOCTOR
- NURSE
- SCIENTIST
- ENGINEER
- PROFESSOR
- TEACHER
- LAWYER_AS_EXPERT
- TECHNICIAN
- CIVIL_SERVANT
- ADMINISTRATOR
- MANAGER
- PARENT
- GUARDIAN
- CAREGIVER
```

Typical weights:

```text
EXPERT / KNOWLEDGE                = TH17
FUNCTIONAL / SOCIAL RESPONSIBILITY = TH5
SPECIAL / EMERGENCY MISSION        = TH33
```

A role can remain structurally L5 while its active TH increases because of emergency, scope, or special mission.

---

## L6 — HUMAN / CIVIL

```text
- INDIVIDUAL
- CITIZEN
- RESIDENT
- STUDENT
- WORKER
- PATIENT
- CUSTOMER
- PASSENGER
- GENERAL_PUBLIC
```

Typical weights:

```text
GENERAL BASELINE       = TH2.5
PASSIVE / MINOR ROLE   = TH1
INDIRECT / TRACE       = TH0.x
```

---

# 2. TH — Responsibility Weight

## 2.1 Baseline Scale

```text
TH33   = SPECIAL MISSION / HIGH MANDATE
TH17   = EXPERT / KNOWLEDGE RESPONSIBILITY
TH5    = FUNCTIONAL / SOCIAL RESPONSIBILITY
TH2.5  = GENERAL HUMAN BASELINE
TH1    = PASSIVE / MINOR ROLE
TH0.x  = INDIRECT / TRACE CONTRIBUTION
```

The scale is inspired by the **structural idea** of honey composition: major components, supporting components, minor components, and trace components. It is a MoonWitness modeling analogy, **not a claim that these TH values are literal chemical percentages or scriptural numbers**.

## 2.2 Dynamic Rule

TH is evaluated **per event**, not permanently attached to a human being.

```text
PERSON
  -> BASE IDENTITY
  -> ACTIVE ROLE
  -> ACTIVE MISSION
  -> ACTIVE MANDATE
  -> EVENT CONTEXT
  -> ACTIVE TH
```

Example:

```text
POLICE / personal purchase / off-duty
  structural role in event = L6 HUMAN/CIVIL
  TH = 2.5 or applicable functional weight

POLICE / lawful enforcement / on-duty
  structural role in event = L4 SPECIAL CIVIL MANDATE
  TH = 33
```

The system MUST store the person's stable identity separately from the event-active role.

---

# 3. MANDATE_SOURCE

Canonical enum:

```text
DIVINE
REVELATORY
RELIGIOUS_STEWARDSHIP
STATE
LEGAL
INSTITUTIONAL
PROFESSIONAL
SOCIAL
PERSONAL
EMERGENCY
DELEGATED
NONE
UNKNOWN
```

Interpretation:

| Source | Meaning |
|---|---|
| `DIVINE` | Framework-level divine commission relation |
| `REVELATORY` | Mission tied to revelation transmission/guidance |
| `RELIGIOUS_STEWARDSHIP` | Religious knowledge/transmission responsibility |
| `STATE` | Mandate established by governing authority |
| `LEGAL` | Mandate established through law/court/legal representation |
| `INSTITUTIONAL` | Organization-specific authority |
| `PROFESSIONAL` | Licensed/recognized professional competence |
| `SOCIAL` | Family/community/social responsibility |
| `PERSONAL` | Individual agency only |
| `EMERGENCY` | Temporary emergency mandate/escalation |
| `DELEGATED` | Explicitly delegated authority |
| `NONE` | No special mandate detected |
| `UNKNOWN` | Evidence insufficient |

A role MAY have more than one mandate source, but each source MUST be explicit and provenance-backed.

---

# 4. MISSION_TYPE

Canonical enum:

```text
REVELATION
BALAGH
GUIDANCE
RELIGIOUS_STEWARDSHIP
GOVERNANCE
ADMINISTRATION
JUSTICE
LEGAL_REPRESENTATION
PROTECTION
DEFENSE
LAW_ENFORCEMENT
SECURITY
EMERGENCY_RESPONSE
HEALING
EDUCATION
RESEARCH
ENGINEERING
PUBLIC_SERVICE
CARE
SOCIAL_SUPPORT
GENERAL_CIVIL
NONE
UNKNOWN
```

Mission type does not determine level alone. Level is resolved from **mission + mandate + active role + event context**.

---

# 5. Authority Dimensions

Every active role SHOULD be decomposed into authority dimensions instead of using one undifferentiated `authority=true` flag.

```text
MESSAGE_AUTHORITY
GUIDANCE_AUTHORITY
ADMIN_AUTHORITY
LEGAL_AUTHORITY
ENFORCEMENT_AUTHORITY
MEDICAL_AUTHORITY
TECHNICAL_AUTHORITY
EDUCATIONAL_AUTHORITY
SOCIAL_AUTHORITY
```

Example:

```text
RASUL
MESSAGE_AUTHORITY       = TRUE
GUIDANCE_AUTHORITY      = TRUE
ADMIN_AUTHORITY         = NOT_IMPLIED
ENFORCEMENT_AUTHORITY   = NOT_IMPLIED

POLICE_ON_DUTY
MESSAGE_AUTHORITY       = FALSE
ADMIN_AUTHORITY         = CONTEXTUAL
ENFORCEMENT_AUTHORITY   = TRUE
LEGAL_AUTHORITY         = CONTEXTUAL

DOCTOR
MEDICAL_AUTHORITY       = TRUE
ENFORCEMENT_AUTHORITY   = FALSE
```

---

# 6. Routing Rules

## 6.1 Message / Guidance Lane

```text
ALLAH
  -> REVELATION
  -> RASUL / NABI
  -> BALAGH / GUIDANCE
  -> HUMAN
```

Direct message/guidance does not equal civil administrative execution.

## 6.2 Religious Stewardship Lane

```text
REVELATION / GUIDANCE
  -> RASUL / NABI
  -> ULAMA / RELIGIOUS SCHOLAR
  -> TEACHING / PRESERVATION / GUIDANCE
  -> COMMUNITY
```

## 6.3 Civil Governance Lane

```text
GUIDANCE / PRINCIPLE
  -> ULUL AMRI / GOVERNMENT
  -> ADMINISTRATION
  -> STATE INSTITUTIONS
  -> PUBLIC SERVICE / OFFICER
  -> HUMAN / CIVIL
```

## 6.4 No-Bypass Rule

For **civil administrative execution**:

```text
RASUL -> PUBLIC ADMIN COMMAND
without administrative/governance interface
= INVALID ROUTE IN THIS ONTOLOGY
```

But:

```text
RASUL -> HUMAN
MESSAGE / BALAGH / GUIDANCE
= VALID
```

This prevents the model from turning prophetic guidance into an undefined “super-administrator” privilege.

---

# 7. MANDATE_SOURCE × MISSION_TYPE × TH Matrix

| Active role | Structural level | Mandate source | Mission type | Typical TH | Notes |
|---|---:|---|---|---:|---|
| Rasul, mission active | L2 | DIVINE / REVELATORY | REVELATION / BALAGH / GUIDANCE | 33 | Divine commission lane |
| Nabi, mission active | L2 | DIVINE / REVELATORY | GUIDANCE | 33 | Same structural domain; function may differ |
| Ulama, knowledge only | L3 | RELIGIOUS_STEWARDSHIP | EDUCATION / GUIDANCE | 17 | Expert knowledge responsibility |
| Ulama, Mission #2 active | L3 | RELIGIOUS_STEWARDSHIP | RELIGIOUS_STEWARDSHIP | 33 | Mission weight; not state power |
| Ulul Amri / government leader | L4 | STATE | GOVERNANCE | 33 | Civil governance mandate |
| Judge, deciding case | L4 | LEGAL / STATE | JUSTICE | 33 | Active adjudicative mandate |
| Prosecutor, active case | L4 | LEGAL / STATE | JUSTICE / LAW_ENFORCEMENT | 33 | Scope constrained by law |
| Police, on duty | L4 | STATE / LEGAL | PROTECTION / LAW_ENFORCEMENT | 33 | Active enforcement mandate |
| Military, active duty | L4 | STATE | DEFENSE / SECURITY | 33 | Mission-specific authority |
| Lawyer, active representation | L4 | LEGAL / DELEGATED | LEGAL_REPRESENTATION | 33 | Case mandate active |
| Lawyer, legal expert only | L5 | PROFESSIONAL | EDUCATION / LEGAL expertise | 17 | No active case mandate |
| Doctor, normal clinical work | L5 | PROFESSIONAL | HEALING | 17 | Expert responsibility |
| Doctor, emergency response | L5 | PROFESSIONAL / EMERGENCY | HEALING / EMERGENCY_RESPONSE | 33 | Same L5 identity domain; escalated TH |
| Scientist / engineer | L5 | PROFESSIONAL | RESEARCH / ENGINEERING | 17 | Expert responsibility |
| Teacher | L5 | PROFESSIONAL / INSTITUTIONAL | EDUCATION | 5–17 | Depends on expertise and event scope |
| Parent / guardian | L5 | SOCIAL | CARE | 5 | Social responsibility |
| General citizen | L6 | PERSONAL | GENERAL_CIVIL | 2.5 | Human baseline |
| Passive witness | L6 | PERSONAL | GENERAL_CIVIL | 1 | Passive/minor participation |
| Trace contributor | L6 | NONE / UNKNOWN | NONE / UNKNOWN | 0.x | Requires causal evidence |

---

# 8. TH Resolution

Do **not** compute active TH from level number.

Bad:

```text
TH = f(level)
```

Required:

```text
TH = resolve(
  active_role,
  mission_type,
  mandate_source,
  authority_scope,
  knowledge_scope,
  event_scope,
  emergency_state,
  causal_involvement
)
```

## 8.1 Canonical Baseline Resolver

```text
if special_mission_or_high_mandate_is_active:
    TH_BASE = 33
else if expert_or_knowledge_responsibility_is_active:
    TH_BASE = 17
else if functional_or_social_responsibility_is_active:
    TH_BASE = 5
else if ordinary_human_agency_is_active:
    TH_BASE = 2.5
else if passive_minor_role_is_supported:
    TH_BASE = 1
else if only_trace_causal_contribution_is_supported:
    TH_BASE = 0.x
else:
    TH_BASE = UNKNOWN
```

The resolver MUST return **evidence and reason codes**, not only a number.

---

# 9. Mizan Input Vector

After TH resolution, Mizan evaluates the wider event vector:

```text
ACTOR
-> BASE_IDENTITY
-> STRUCTURAL_LEVEL
-> ACTIVE_ROLE
-> MISSION_TYPE
-> MANDATE_SOURCE
-> AUTHORITY_SCOPE
-> KNOWLEDGE_SCOPE
-> TH
-> SCOPE
-> ACCESS / HIZAB
-> INTENT
-> ACTION
-> IMPACT
-> CONTEXT
-> EVIDENCE
-> PROVENANCE
-> MIZAN ANALYSIS
-> LEDGER
```

TH MUST NOT replace intent, evidence, impact, or boundary analysis.

---

# 10. Boundary / Conflict Rules

The following are independent boundaries:

```text
RELIGIOUS_AUTHORITY != POLICE_AUTHORITY
POLICE_AUTHORITY    != MEDICAL_AUTHORITY
MEDICAL_AUTHORITY   != JUDICIAL_AUTHORITY
EXPERT_KNOWLEDGE    != ADMIN_COMMAND
GUIDANCE_AUTHORITY  != ENFORCEMENT_AUTHORITY
SAME_TH              != SAME_BOUNDARY
```

A role MUST NOT borrow another role's authority simply because both have TH33.

---

# 11. Event-State Examples

## Case A — Police Off Duty

```text
base_identity       = HUMAN
professional_role   = POLICE
active_event_role   = CIVIL
mandate_active      = FALSE
level               = L6 for this event
TH                  = 2.5
```

## Case B — Police On Duty

```text
active_event_role   = POLICE
mandate_source      = STATE / LEGAL
mission_type        = LAW_ENFORCEMENT
level               = L4
TH                  = 33
```

## Case C — Doctor Normal Practice

```text
active_event_role   = DOCTOR
mandate_source      = PROFESSIONAL
mission_type        = HEALING
level               = L5
TH                  = 17
```

## Case D — Doctor Emergency Mission

```text
active_event_role   = DOCTOR
mandate_source      = PROFESSIONAL + EMERGENCY
mission_type        = HEALING + EMERGENCY_RESPONSE
level               = L5
TH                  = 33
```

## Case E — Ulama Teaching

```text
active_event_role   = RELIGIOUS_SCHOLAR
mandate_source      = RELIGIOUS_STEWARDSHIP
mission_type        = EDUCATION / GUIDANCE
level               = L3
TH                  = 17
```

## Case F — Ulama Mission #2 Active

```text
active_event_role   = RELIGIOUS_STEWARD
mandate_source      = RELIGIOUS_STEWARDSHIP
mission_type        = RELIGIOUS_STEWARDSHIP
level               = L3
TH                  = 33
```

## Case G — Lawyer Without Active Representation

```text
active_event_role   = LEGAL_EXPERT
mandate_source      = PROFESSIONAL
level               = L5
TH                  = 17
```

## Case H — Lawyer With Active Case Mandate

```text
active_event_role   = ADVOCATE
mandate_source      = LEGAL + DELEGATED
mission_type        = LEGAL_REPRESENTATION
level               = L4
TH                  = 33
```

---

# 12. Evidence Discipline

Every assignment MUST have an epistemic status compatible with the main MoonWitness paper:

```text
EXPLICIT
DERIVED
HISTORICAL
CLAIMED
INTERPRETIVE
FRAMEWORK
UNKNOWN
```

The L0–L6 operational mapping, TH scale, mission enums, and routing rules are **FRAMEWORK constructs unless separately grounded with explicit evidence**.

The engine MUST preserve this distinction in storage and output.

---

# 13. Machine Contract

A compatible event-role record SHOULD contain at least:

```json
{
  "actor_id": "actor:example",
  "event_id": "event:example",
  "base_identity": "HUMAN",
  "structural_level": "L4",
  "active_role": "POLICE",
  "mission_types": ["LAW_ENFORCEMENT", "PROTECTION"],
  "mandate_sources": ["STATE", "LEGAL"],
  "authority_dimensions": ["ENFORCEMENT_AUTHORITY"],
  "th": {
    "value": 33,
    "class": "SPECIAL_MISSION_HIGH_MANDATE",
    "reason_codes": ["ACTIVE_STATE_MANDATE", "LAW_ENFORCEMENT_EVENT"]
  },
  "epistemic_status": "FRAMEWORK",
  "evidence_refs": []
}
```

---

# 14. Invariants

An implementation MUST enforce:

```text
INV-001  L0 ALLAH is not numerically scored.
INV-002  L1 relation/designation/support is not administrative rank.
INV-003  TH is event-based, not a permanent human value.
INV-004  TH33 does not imply prophetic identity.
INV-005  TH33 does not merge authority domains.
INV-006  Civil administrative execution must resolve through a civil mandate path.
INV-007  Rasul -> Human direct guidance is distinct from civil admin execution.
INV-008  Professional emergency escalation may raise TH without changing stable identity.
INV-009  Every TH assignment requires reason codes and evidence/provenance.
INV-010  Unknown evidence must remain UNKNOWN; the engine must not fabricate authority or mission.
INV-011  Mizan analysis stops short of claiming God's final judgment.
INV-012  Framework assertions must remain distinguishable from explicit textual assertions.
```

---

# 15. Next Implementation Step

After this blueprint is accepted, implement:

1. `RoleDefinition`
2. `MandateSource`
3. `MissionType`
4. `AuthorityDimension`
5. `THResolution`
6. `RoleTransition`
7. `RoutingValidation`
8. `EvidenceBinding`
9. `MizanEventVector`
10. test fixtures for at least 10 real role/event cases

The machine-readable companion specification is stored in:

> `mizan-governance-th.v1.json`

---

## Final Compression

```text
ALLAH
  -> REVELATION / PRINCIPLE
  -> DIVINE COMMISSION
  -> RELIGIOUS STEWARDSHIP
  -> GOVERNANCE / CIVIL MANDATE
  -> PROFESSIONAL / PUBLIC SERVICE
  -> HUMAN / CIVIL

LEVEL = WHERE / WHAT DOMAIN IS ACTIVE
ROLE = WHAT FUNCTION IS ACTIVE
MISSION = WHAT OBJECTIVE IS ENTRUSTED
MANDATE = WHERE AUTHORITY COMES FROM
TH = HOW HEAVY THE EVENT RESPONSIBILITY IS
MIZAN = HOW THE RELEVANT EVIDENCE AND FACTORS ARE WEIGHED
LEDGER = WHAT IS RECORDED WITH PROVENANCE
```
