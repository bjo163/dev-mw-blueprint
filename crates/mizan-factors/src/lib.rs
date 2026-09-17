use mizan_model::{
    AnalyticalFactorsInput, CausalContributionLevel, ContextState, ImpactLevel, IntentState,
    ReasonCode, ScopeLevel, ThClass,
};
use serde::{Deserialize, Serialize};

pub const ARI_CALIBRATION_VERSION: &str = "ARI-0.1.0";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactorVector {
    pub th_normalized: Option<f64>,
    pub intent: Option<f64>,
    pub impact: Option<f64>,
    pub scope: Option<f64>,
    pub context_multiplier: Option<f64>,
    pub causal_contribution: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactorAssessment {
    pub calibration_version: String,
    pub vector: FactorVector,
    pub complete: bool,
    pub analytical_responsibility_index: Option<f64>,
    pub reason_codes: Vec<ReasonCode>,
    pub final_moral_or_divine_verdict: bool,
}

pub fn assess_factors(th: &ThClass, factors: &AnalyticalFactorsInput) -> FactorAssessment {
    let vector = FactorVector {
        th_normalized: th.numeric_value().map(|v| (v / 33.0).clamp(0.0, 1.0)),
        intent: score_intent(factors.intent),
        impact: score_impact(factors.impact),
        scope: score_scope(factors.scope),
        context_multiplier: score_context(factors.context),
        causal_contribution: score_causal(factors.causal_contribution),
    };

    let complete = vector.th_normalized.is_some()
        && vector.intent.is_some()
        && vector.impact.is_some()
        && vector.scope.is_some()
        && vector.context_multiplier.is_some()
        && vector.causal_contribution.is_some();

    let analytical_responsibility_index = if complete {
        let base = vector.th_normalized.unwrap() * 0.30
            + vector.intent.unwrap() * 0.20
            + vector.impact.unwrap() * 0.20
            + vector.scope.unwrap() * 0.10
            + vector.causal_contribution.unwrap() * 0.20;
        Some((base * vector.context_multiplier.unwrap()).clamp(0.0, 1.0) * 100.0)
    } else {
        None
    };

    let reason_codes = if complete {
        vec![
            ReasonCode("ANALYTICAL_FACTOR_VECTOR_COMPLETE".into()),
            ReasonCode("ARI_COMPUTED".into()),
            ReasonCode("ARI_NOT_MORAL_OR_DIVINE_VERDICT".into()),
        ]
    } else {
        vec![
            ReasonCode("ANALYTICAL_FACTOR_VECTOR_INCOMPLETE".into()),
            ReasonCode("ARI_NOT_COMPUTED_INCOMPLETE_FACTORS".into()),
        ]
    };

    FactorAssessment {
        calibration_version: ARI_CALIBRATION_VERSION.into(),
        vector,
        complete,
        analytical_responsibility_index,
        reason_codes,
        final_moral_or_divine_verdict: false,
    }
}

fn score_intent(value: IntentState) -> Option<f64> {
    match value {
        IntentState::Accidental => Some(0.15),
        IntentState::Negligent => Some(0.45),
        IntentState::Knowing => Some(0.75),
        IntentState::Deliberate => Some(1.00),
        IntentState::Unknown => None,
    }
}

fn score_impact(value: ImpactLevel) -> Option<f64> {
    match value {
        ImpactLevel::None => Some(0.00),
        ImpactLevel::Low => Some(0.25),
        ImpactLevel::Moderate => Some(0.50),
        ImpactLevel::High => Some(0.75),
        ImpactLevel::Severe => Some(1.00),
        ImpactLevel::Unknown => None,
    }
}

fn score_scope(value: ScopeLevel) -> Option<f64> {
    match value {
        ScopeLevel::Individual => Some(0.15),
        ScopeLevel::Household => Some(0.25),
        ScopeLevel::Local => Some(0.40),
        ScopeLevel::Institutional => Some(0.55),
        ScopeLevel::Regional => Some(0.70),
        ScopeLevel::National => Some(0.85),
        ScopeLevel::Transnational => Some(1.00),
        ScopeLevel::Unknown => None,
    }
}

fn score_context(value: ContextState) -> Option<f64> {
    match value {
        ContextState::Constrained => Some(0.65),
        ContextState::Normal => Some(1.00),
        ContextState::ElevatedDuty => Some(1.10),
        ContextState::Emergency => Some(1.05),
        ContextState::Unknown => None,
    }
}

fn score_causal(value: CausalContributionLevel) -> Option<f64> {
    match value {
        CausalContributionLevel::Trace => Some(0.10),
        CausalContributionLevel::Minor => Some(0.25),
        CausalContributionLevel::Material => Some(0.50),
        CausalContributionLevel::Major => Some(0.75),
        CausalContributionLevel::Direct => Some(1.00),
        CausalContributionLevel::Unknown => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mizan_model::FactorEvidenceBindings;

    #[test]
    fn complete_vector_computes_ari() {
        let factors = AnalyticalFactorsInput {
            intent: IntentState::Deliberate,
            impact: ImpactLevel::High,
            scope: ScopeLevel::Local,
            context: ContextState::Normal,
            causal_contribution: CausalContributionLevel::Direct,
            evidence_bindings: FactorEvidenceBindings::default(),
        };
        let result = assess_factors(&ThClass::Functional5, &factors);
        assert!(result.complete);
        assert!(result.analytical_responsibility_index.is_some());
        assert!(!result.final_moral_or_divine_verdict);
    }

    #[test]
    fn unknown_factor_blocks_ari() {
        let factors = AnalyticalFactorsInput {
            intent: IntentState::Unknown,
            impact: ImpactLevel::High,
            scope: ScopeLevel::Local,
            context: ContextState::Normal,
            causal_contribution: CausalContributionLevel::Direct,
            evidence_bindings: FactorEvidenceBindings::default(),
        };
        let result = assess_factors(&ThClass::Functional5, &factors);
        assert!(!result.complete);
        assert_eq!(result.analytical_responsibility_index, None);
    }
}
