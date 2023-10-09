use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Enumeration<T: Copy> {
    pub values: Vec<T>,
}

impl Constraint for Enumeration<i64> {
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

impl Constraint for Enumeration<f64> {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_f64() && !self.values.contains(&v.as_f64().unwrap()) => {
                Err(anyhow!(format!(
                    "value {} is not valid value ({})",
                    v, "Enum of Float"
                )))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::number::enumeration::Enumeration;
    use crate::core::constraint::Constraint;
    use serde_json::{Number, Value};

    #[test]
    fn test_enumeration_i64() {
        let constraint = Enumeration { values: vec![1, 2] };

        let value = Value::Number(1.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_err());
    }

    #[test]
    fn test_enumeration_f64() {
        let constraint = Enumeration {
            values: vec![1.1, 2.5],
        };

        let value = Value::Number(Number::from_f64(1.1).unwrap());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(3.0).unwrap());
        assert!(constraint.validate(&value).is_err());
    }
}
