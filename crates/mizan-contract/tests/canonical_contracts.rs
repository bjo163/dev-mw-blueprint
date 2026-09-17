use mizan_contract::{Contract, ContractKind};

#[test]
fn canonical_contract_chain_validates() {
    let base_raw = include_str!("../../../mizan-governance-th.v1.json");
    let extension_raw = include_str!("../../../mizan-governance-th.v1.1.json");
    let analytical_raw = include_str!("../../../mizan-analytical-factors.v1.2.json");

    let base = Contract::from_json_str(base_raw).expect("v1 base contract must validate");
    assert_eq!(base.kind(), ContractKind::Base);
    assert_eq!(base.schema_version(), "1.0.0");

    let extension = Contract::from_json_str(extension_raw).expect("v1.1 extension contract must validate");
    assert_eq!(extension.kind(), ContractKind::Extension);
    assert_eq!(extension.schema_version(), "1.1.0");
    assert_eq!(extension.extends(), Some("mizan-governance-th.v1.json"));

    let analytical = Contract::from_json_str(analytical_raw).expect("v1.2 analytical extension must validate");
    assert_eq!(analytical.kind(), ContractKind::AnalyticalExtension);
    assert_eq!(analytical.schema_version(), "1.2.0");
    assert_eq!(analytical.extends(), Some("mizan-governance-th.v1.1.json"));

    let levels = base.raw()["levels"].as_array().expect("levels array");
    assert_eq!(levels.len(), 7, "L0-L6 must remain complete");

    let invariants = extension.raw()["invariants_add"].as_array().expect("extension invariants");
    assert!(invariants.iter().any(|v| {
        v.get("id").and_then(|x| x.as_str()) == Some("INV-NAS-002")
    }), "COUNSEL_AUTHORITY boundary invariant must remain present");

    let analytical_invariants = analytical.raw()["invariants_add"]
        .as_array()
        .expect("analytical invariants");
    assert!(analytical_invariants.iter().any(|v| {
        v.get("id").and_then(|x| x.as_str()) == Some("INV-ARI-002")
    }), "ARI must remain explicitly non-divine and non-final");
}
