use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Enumeration {
    pub values: Vec<i64>,
}

impl Constraint for Enumeration {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_i64() && !self.values.contains(&v.as_i64().unwrap()) => {
                Err(anyhow!(format!(
                    "value {} is not valid value ({})",
                    v, "Enum of Integer"
                )))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::integer::enumeration::Enumeration;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_enumeration() {
        let constraint = Enumeration { values: vec![1, 2] };

        let value = Value::Number(1.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_err());
    }
}
