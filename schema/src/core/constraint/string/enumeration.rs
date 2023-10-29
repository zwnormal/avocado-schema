use crate::core::constraint::Constraint;
use crate::core::value::FieldValue;
use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct Enumeration {
    pub values: Vec<String>,
}

impl Constraint for Enumeration {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::String(v) if !self.values.contains(v) => Err(anyhow!(format!(
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
    use crate::core::value::FieldValue;

    #[test]
    fn test_enumeration() {
        let constraint = Enumeration {
            values: vec!["China".to_string(), "Australia".to_string()],
        };

        let value = FieldValue::String("China".to_string());
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::String("United States".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
