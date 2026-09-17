use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralLevel {
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipDomain {
    FamilyKinship,
    CommunitySocial,
    Professional,
    Governance,
    Legal,
    Religious,
    GeneralCivil,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityType {
    Balagh,
    Guidance,
    Nasihah,
    CorrectiveGuidance,
    Administration,
    Justice,
    LawEnforcement,
    Protection,
    Defense,
    Healing,
    Education,
    Research,
    Engineering,
    Care,
    PublicService,
    GeneralCivil,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActiveRole {
    Rasul,
    Nabi,
    Ulama,
    ReligiousScholar,
    UlulAmri,
    GovernmentLeader,
    Judge,
    Prosecutor,
    Police,
    Military,
    Advocate,
    Doctor,
    Nurse,
    Scientist,
    Engineer,
    Teacher,
    Parent,
    Guardian,
    Caregiver,
    Citizen,
    Human,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionType {
    Revelation,
    Balagh,
    Guidance,
    ReligiousStewardship,
    Governance,
    Administration,
    Justice,
    LegalRepresentation,
    Protection,
    Defense,
    LawEnforcement,
    Security,
    EmergencyResponse,
    Healing,
    Education,
    Research,
    Engineering,
    PublicService,
    Care,
    SocialSupport,
    GeneralCivil,
    None,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MandateSource {
    Divine,
    Revelatory,
    ReligiousStewardship,
    State,
    Legal,
    Institutional,
    Professional,
    Social,
    Personal,
    Emergency,
    Delegated,
    None,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorityDimension {
    Message,
    Guidance,
    Counsel,
    Admin,
    Legal,
    Enforcement,
    Medical,
    Technical,
    Educational,
    Social,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThClass {
    SpecialMission33,
    Expert17,
    Functional5,
    HumanBaseline2_5,
    Passive1,
    Trace(f64),
    Unknown,
}

impl ThClass {
    pub fn numeric_value(&self) -> Option<f64> {
        match self {
            Self::SpecialMission33 => Some(33.0),
            Self::Expert17 => Some(17.0),
            Self::Functional5 => Some(5.0),
            Self::HumanBaseline2_5 => Some(2.5),
            Self::Passive1 => Some(1.0),
            Self::Trace(value) => Some(*value),
            Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteKind {
    MessageGuidance,
    ReligiousStewardship,
    CivilGovernance,
    ProfessionalService,
    FamilyGuidance,
    CrossLevelCounsel,
    GeneralCivil,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub id: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasonCode(pub String);

/// Raw event input. Structural level and effective authority are intentionally
/// not trusted from callers; the engine resolves them from role + context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MizanInput {
    pub actor_id: String,
    pub active_role: ActiveRole,
    pub relationship_domains: Vec<RelationshipDomain>,
    pub activities: Vec<ActivityType>,
    pub mission_types: Vec<MissionType>,
    pub mandate_sources: Vec<MandateSource>,
    #[serde(default)]
    pub requested_authority_dimensions: Vec<AuthorityDimension>,
    pub route: RouteKind,
    #[serde(default)]
    pub mandate_active: bool,
    #[serde(default)]
    pub emergency_state: bool,
    #[serde(default)]
    pub expert_knowledge_active: bool,
    #[serde(default)]
    pub functional_responsibility_active: bool,
    #[serde(default)]
    pub passive_role: bool,
    pub trace_contribution: Option<f64>,
    #[serde(default)]
    pub evidence: Vec<EvidenceRef>,
}

/// Resolved event consumed by TH/routing. `level` and
/// `authority_dimensions` are engine-derived fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MizanEvent {
    pub actor_id: String,
    pub level: StructuralLevel,
    pub active_role: ActiveRole,
    pub relationship_domains: Vec<RelationshipDomain>,
    pub activities: Vec<ActivityType>,
    pub mission_types: Vec<MissionType>,
    pub mandate_sources: Vec<MandateSource>,
    pub authority_dimensions: Vec<AuthorityDimension>,
    pub route: RouteKind,
    pub mandate_active: bool,
    pub emergency_state: bool,
    pub expert_knowledge_active: bool,
    pub functional_responsibility_active: bool,
    pub passive_role: bool,
    pub trace_contribution: Option<f64>,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub valid: bool,
    pub reason_codes: Vec<ReasonCode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MizanResult {
    pub level: StructuralLevel,
    pub th: ThClass,
    pub routing: RoutingDecision,
    pub reason_codes: Vec<ReasonCode>,
}
