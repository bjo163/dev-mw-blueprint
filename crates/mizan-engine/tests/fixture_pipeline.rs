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
    level: String,
    th: f64,
    route_valid: bool,
    authority_valid: bool,
    structurally_valid: bool,
}

#[test]
fn full_pipeline_fixtures_match_contract() {
    let raw = include_str!("../../../mizan-engine.fixtures.v1.json");
    let fixtures: FixtureSet = serde_json::from_str(raw).expect("valid fixture JSON");
    assert!(fixtures.cases.len() >= 10, "baseline requires >= 10 pipeline cases");

    for case in fixtures.cases {
        let result = evaluate_input(&case.input);
        let actual_level = format!("{:?}", result.resolved_event.level);
        let actual_th = result
            .result
            .th
            .numeric_value()
            .unwrap_or_else(|| panic!("{} {} resolved unknown TH", case.id, case.name));

        assert_eq!(actual_level, case.expected.level, "{} {} level", case.id, case.name);
        assert!((actual_th - case.expected.th).abs() < f64::EPSILON, "{} {} TH", case.id, case.name);
        assert_eq!(result.result.routing.valid, case.expected.route_valid, "{} {} route", case.id, case.name);
        assert_eq!(result.authority_valid, case.expected.authority_valid, "{} {} authority", case.id, case.name);
        assert_eq!(result.structurally_valid, case.expected.structurally_valid, "{} {} structural", case.id, case.name);
        assert!(!result.final_divine_judgment_computed, "{} must never compute final divine judgment", case.id);
    }
}
