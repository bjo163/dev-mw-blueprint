use mizan_engine::evaluate_input;
use mizan_model::MizanInput;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FixtureSet {
    cases: Vec<FixtureCase>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    id: String,
    name: String,
    input: MizanInput,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct Expected {
    ari_computed: bool,
    evidence_supported: bool,
    strength: String,
    #[serde(default)]
    provenance_complete: Option<bool>,
    min_ari: Option<f64>,
    max_ari: Option<f64>,
}

#[test]
fn analytical_fixtures_match_v12_contract() {
    let raw = include_str!("../../../mizan-analytical.fixtures.v1.2.json");
    let fixtures: FixtureSet = serde_json::from_str(raw).expect("valid analytical fixture JSON");
    assert!(fixtures.cases.len() >= 10, "analytical baseline requires >= 10 cases");

    for case in fixtures.cases {
        let calculation = evaluate_input(&case.input);
        let ari = calculation.factor_assessment.analytical_responsibility_index;
        assert_eq!(
            ari.is_some(),
            case.expected.ari_computed,
            "{} {} ARI computed state",
            case.id,
            case.name
        );
        assert_eq!(
            calculation.analytical_index_evidence_sufficient,
            case.expected.evidence_supported,
            "{} {} evidence-supported state",
            case.id,
            case.name
        );
        assert_eq!(
            format!("{:?}", calculation.evidence_assessment.strength),
            case.expected.strength,
            "{} {} evidence strength",
            case.id,
            case.name
        );

        if let Some(expected_provenance) = case.expected.provenance_complete {
            assert_eq!(
                calculation.evidence_assessment.provenance_complete,
                expected_provenance,
                "{} {} provenance completeness",
                case.id,
                case.name
            );
        }

        match (ari, case.expected.min_ari, case.expected.max_ari) {
            (Some(value), Some(min), Some(max)) => assert!(
                value >= min && value <= max,
                "{} {} ARI {} outside [{}, {}]",
                case.id,
                case.name,
                value,
                min,
                max
            ),
            (None, None, None) => {}
            other => panic!("{} {} inconsistent ARI expectation: {:?}", case.id, case.name, other),
        }

        assert!(
            !calculation.factor_assessment.final_moral_or_divine_verdict,
            "{} must never produce final moral/divine verdict",
            case.id
        );
        assert!(
            !calculation.final_divine_judgment_computed,
            "{} must never compute final divine judgment",
            case.id
        );
    }
}
