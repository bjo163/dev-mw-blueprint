use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("invalid JSON contract: {0}")]
    Json(#[from] serde_json::Error),
    #[error("missing required contract field: {0}")]
    MissingField(&'static str),
    #[error("unsupported contract version: {0}")]
    UnsupportedVersion(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractKind {
    Base,
    Extension,
    AnalyticalExtension,
}

#[derive(Debug, Clone)]
pub struct Contract {
    raw: Value,
    kind: ContractKind,
}

impl Contract {
    pub fn from_json_str(input: &str) -> Result<Self, ContractError> {
        let raw: Value = serde_json::from_str(input)?;
        let version = raw
            .get("schema_version")
            .and_then(Value::as_str)
            .ok_or(ContractError::MissingField("schema_version"))?;

        if version != "1.0.0" && version != "1.1.0" && version != "1.2.0" {
            return Err(ContractError::UnsupportedVersion(version.to_owned()));
        }

        let kind = match version {
            "1.2.0" => {
                validate_analytical_extension(&raw)?;
                ContractKind::AnalyticalExtension
            }
            _ if raw.get("extends").is_some() => {
                validate_extension(&raw)?;
                ContractKind::Extension
            }
            _ => {
                validate_base(&raw)?;
                ContractKind::Base
            }
        };

        Ok(Self { raw, kind })
    }

    pub fn schema_version(&self) -> &str {
        self.raw["schema_version"].as_str().expect("validated schema_version")
    }

    pub fn kind(&self) -> ContractKind {
        self.kind
    }

    pub fn extends(&self) -> Option<&str> {
        self.raw.get("extends").and_then(Value::as_str)
    }

    pub fn raw(&self) -> &Value {
        &self.raw
    }
}

fn validate_base(raw: &Value) -> Result<(), ContractError> {
    for field in ["levels", "th_classes", "mandate_sources", "mission_types", "invariants"] {
        if raw.get(field).is_none() {
            return Err(ContractError::MissingField(field));
        }
    }
    Ok(())
}

fn validate_extension(raw: &Value) -> Result<(), ContractError> {
    for field in ["extends", "relationship_domains_add", "activity_domains_add", "invariants_add"] {
        if raw.get(field).is_none() {
            return Err(ContractError::MissingField(field));
        }
    }
    Ok(())
}

fn validate_analytical_extension(raw: &Value) -> Result<(), ContractError> {
    for field in [
        "extends",
        "evidence_model",
        "analytical_factors",
        "ari_calibration",
        "invariants_add",
    ] {
        if raw.get(field).is_none() {
            return Err(ContractError::MissingField(field));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_base_contract_without_required_fields() {
        let err = Contract::from_json_str(r#"{"schema_version":"1.0.0"}"#).unwrap_err();
        assert!(matches!(err, ContractError::MissingField("levels")));
    }

    #[test]
    fn recognizes_extension_contract() {
        let input = r#"{
          "schema_version":"1.1.0",
          "extends":"base.json",
          "relationship_domains_add":[],
          "activity_domains_add":[],
          "invariants_add":[]
        }"#;
        let contract = Contract::from_json_str(input).unwrap();
        assert_eq!(contract.kind(), ContractKind::Extension);
        assert_eq!(contract.extends(), Some("base.json"));
    }

    #[test]
    fn recognizes_analytical_extension_contract() {
        let input = r#"{
          "schema_version":"1.2.0",
          "extends":"v1.1.json",
          "evidence_model":{},
          "analytical_factors":{},
          "ari_calibration":{},
          "invariants_add":[]
        }"#;
        let contract = Contract::from_json_str(input).unwrap();
        assert_eq!(contract.kind(), ContractKind::AnalyticalExtension);
    }
}
