# MIZAN — FAMILY / KINSHIP + NASIHAH DOMAIN

**Status:** Baseline extension v1.1  
**Repository:** `bjo163/dev-mw-blueprint`  
**Target:** MoonWitness / Mizan  
**Purpose:** Add the missing relational family domain and the cross-level corrective-guidance activity domain without confusing relationship, knowledge, advice, or administrative authority.

---

## 0. Core Distinction

MoonWitness MUST distinguish:

```text
RELATIONSHIP DOMAIN != ACTIVITY DOMAIN
FAMILY SENIORITY    != ABSOLUTE AUTHORITY
KNOWLEDGE            != AGE
GUIDANCE             != ADMINISTRATIVE COMMAND
ADVICE               != COERCION
CORRECTION           != ENFORCEMENT
```

`FAMILY / KINSHIP` answers: **what human relationship exists between actor and target?**  
`NASIHAH / CORRECTIVE GUIDANCE` answers: **what guidance activity is occurring?**

The two may occur together, but neither implies the other.

---

# 1. FAMILY / KINSHIP RESPONSIBILITY DOMAIN

Canonical id:

```text
FAMILY_KINSHIP
```

Canonical relations:

```text
PARENT
CHILD
SPOUSE
SIBLING
GUARDIAN
DEPENDENT
RELATIVE
HOUSEHOLD_MEMBER
CAREGIVER
```

Core functions:

```text
CARE
PROTECTION
MAINTENANCE
EDUCATION
GUIDANCE
WARNING
SUPPORT
HOUSEHOLD_RESPONSIBILITY
```

Typical mandate sources:

```text
SOCIAL
FAMILY
GUARDIANSHIP
DELEGATED
```

Typical TH:

```text
ordinary family / care responsibility = TH5
expert knowledge active               = TH17
special mission / exceptional mandate = TH33
```

TH remains event-based. Being a parent, child, spouse, or relative does not permanently assign a high TH to every action.

---

# 2. NASIHAH / COUNSEL / CORRECTIVE GUIDANCE DOMAIN

Canonical id:

```text
NASIHAH_CORRECTIVE_GUIDANCE
```

Framework meaning:

```text
ADVISE
WARN
REMIND
CORRECT
COUNSEL
GUIDE
```

This is a **MoonWitness activity-domain label**. It MUST NOT be represented as a claim that every source passage being mapped literally contains the Arabic word `nasihah`.

The domain is useful when the event contains respectful or reasoned correction across an existing relationship, including:

```text
CHILD -> PARENT
PARENT -> CHILD
SCHOLAR -> LEADER
CITIZEN -> AUTHORITY
DOCTOR -> PATIENT
TEACHER -> STUDENT
FRIEND -> FRIEND
JUNIOR -> SENIOR
SENIOR -> JUNIOR
```

---

# 3. Qur'anic Anchor Case — Ibrahim and His Father

QS 19:41–45 provides a strong anchor case for the distinction between **family relationship** and **corrective guidance activity**.

Framework mapping:

```text
ACTOR_RELATIONSHIP = CHILD
TARGET_RELATIONSHIP = PARENT
RELATION_DOMAIN = FAMILY_KINSHIP
ACTIVITY_DOMAIN = NASIHAH_CORRECTIVE_GUIDANCE
FUNCTIONS = GUIDANCE + WARNING + CORRECTION
ADMIN_COMMAND = FALSE
COERCIVE_AUTHORITY = NOT_IMPLIED
```

The framework should therefore be capable of representing:

```text
BIOLOGICAL / FAMILY SENIORITY:
PARENT -> CHILD

GUIDANCE EVENT DIRECTION:
CHILD -> PARENT
```

without concluding that the child has become the parent's administrative superior.

---

# 4. Authority Boundary

The following invariants are mandatory:

```text
FAMILY_RELATION != GOVERNMENT_AUTHORITY
FAMILY_SENIORITY != INFALLIBILITY
JUNIOR_STATUS != NO_RIGHT_TO_ADVISE
ADVICE != ENFORCEMENT
GUIDANCE != ADMIN_COMMAND
KNOWLEDGE_ADVANTAGE != TOTAL_AUTHORITY
```

A person may possess relevant knowledge for an event while remaining junior in age, family order, job rank, or civil hierarchy.

---

# 5. New Authority Dimension

Add:

```text
COUNSEL_AUTHORITY
```

Meaning:

> Evidence-supported standing to advise, warn, remind, or offer corrective guidance in the relevant context.

`COUNSEL_AUTHORITY` does NOT imply:

```text
ADMIN_AUTHORITY
LEGAL_AUTHORITY
ENFORCEMENT_AUTHORITY
COERCIVE_AUTHORITY
```

---

# 6. New Mission / Activity Types

Add canonical values:

```text
FAMILY_CARE
FAMILY_GUIDANCE
COUNSEL
WARNING
CORRECTIVE_GUIDANCE
REMINDER
```

These values describe the event function. They do not automatically determine structural level.

---

# 7. New Mandate Sources

Add canonical values:

```text
FAMILY
GUARDIANSHIP
```

These coexist with the existing `SOCIAL` and `DELEGATED` sources.

---

# 8. Event Vector Extension

Add relationship-aware fields:

```text
RELATION_DOMAIN
ACTOR_RELATION_TO_TARGET
TARGET_RELATION_TO_ACTOR
ACTIVITY_DOMAIN
COUNSEL_SCOPE
CORRECTION_TARGET
RESPECT_BOUNDARY
COERCION_PRESENT
```

Recommended event flow:

```text
ACTOR
-> TARGET
-> RELATION_DOMAIN
-> ACTIVE_ROLE
-> ACTIVITY_DOMAIN
-> KNOWLEDGE_SCOPE
-> COUNSEL_AUTHORITY
-> MANDATE_SOURCE
-> TH
-> ACTION
-> METHOD
-> INTENT
-> IMPACT
-> EVIDENCE
-> MIZAN
```

---

# 9. Routing

## Family Guidance Lane

```text
KNOWLEDGE / CONCERN
  -> FAMILY RELATION
  -> COUNSEL / WARNING / CORRECTIVE GUIDANCE
  -> FAMILY MEMBER
```

## Cross-Level Counsel Lane

```text
RELEVANT KNOWLEDGE / EVIDENCE
  -> COUNSEL
  -> HIGHER / LOWER / PEER ROLE
```

This lane does not transfer the target's administrative authority to the adviser.

Examples:

```text
CHILD -> PARENT           = VALID COUNSEL ROUTE
CITIZEN -> LEADER         = VALID COUNSEL ROUTE
DOCTOR -> LEADER          = VALID MEDICAL COUNSEL ROUTE
SCHOLAR -> GOVERNMENT     = VALID GUIDANCE / COUNSEL ROUTE

COUNSEL -> SEIZE ADMIN COMMAND = INVALID WITHOUT SEPARATE MANDATE
```

---

# 10. TH Guidance

The domain itself does not force TH33.

```text
ordinary family advice                    -> TH5 baseline candidate
specialized / knowledge-backed counsel    -> TH17 candidate
formal high-stakes special mission        -> TH33 candidate
ordinary human comment without duty       -> TH2.5 candidate
```

The final TH resolver still evaluates event scope, knowledge, mandate, causal involvement, risk, and evidence.

---

# 11. New Invariants

```text
INV-FAM-001  FAMILY/KINSHIP is a relationship domain, not a superiority ladder.
INV-FAM-002  Parent status does not make every statement epistemically correct.
INV-FAM-003  Child/junior status does not prohibit evidence-based counsel.
INV-NAS-001  NASIHAH/CORRECTIVE_GUIDANCE is an activity domain, not civil command.
INV-NAS-002  COUNSEL_AUTHORITY does not imply ADMIN or ENFORCEMENT authority.
INV-NAS-003  Advice can route upward, downward, or laterally across structural levels.
INV-NAS-004  Respect and correction can coexist in the same event.
INV-NAS-005  The label NASIHAH is framework terminology unless explicitly present in the cited source.
```

---

# 12. Final Placement

The expanded domain model now contains, among others:

```text
DIVINE
REVELATION
RELIGIOUS
GOVERNANCE
LEGAL / SECURITY
PROFESSIONAL
FAMILY / KINSHIP
COMMUNITY / SOCIAL
GENERAL CIVIL

CROSS-DOMAIN ACTIVITY:
NASIHAH / COUNSEL / CORRECTIVE GUIDANCE
```

The critical distinction is:

> **FAMILY describes the relationship. NASIHAH describes the corrective-guidance activity. Neither automatically grants administrative power.**
