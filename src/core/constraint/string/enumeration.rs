use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Enumeration {
    pub values: Vec<String>,
}

impl Constraint for Enumeration {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::String(v) if !self.values.contains(v) => Err(anyhow!(format!(
                "value {} is not valid value ({})",
                v, "Enum of String"
            ))),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::enumeration::Enumeration;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_enumeration() {
        let constraint = Enumeration {
            values: vec!["China".to_string(), "Australia".to_string()],
        };

        let value = Value::String("China".to_string());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("United States".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
