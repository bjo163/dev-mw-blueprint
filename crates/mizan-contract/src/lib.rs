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

#[derive(Debug, Clone)]
pub struct Contract {
    raw: Value,
}

impl Contract {
    pub fn from_json_str(input: &str) -> Result<Self, ContractError> {
        let raw: Value = serde_json::from_str(input)?;
        let version = raw
            .get("schema_version")
            .and_then(Value::as_str)
            .ok_or(ContractError::MissingField("schema_version"))?;

        if version != "1.0.0" && version != "1.1.0" {
            return Err(ContractError::UnsupportedVersion(version.to_owned()));
        }

        for field in ["levels", "th_classes", "mandate_sources", "mission_types", "invariants"] {
            if raw.get(field).is_none() {
                return Err(ContractError::MissingField(field));
            }
        }

        Ok(Self { raw })
    }

    pub fn schema_version(&self) -> &str {
        self.raw["schema_version"].as_str().expect("validated schema_version")
    }

    pub fn raw(&self) -> &Value {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_contract_without_required_fields() {
        let err = Contract::from_json_str(r#"{"schema_version":"1.1.0"}"#).unwrap_err();
        assert!(matches!(err, ContractError::MissingField("levels")));
    }
}
