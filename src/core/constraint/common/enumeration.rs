use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enumeration<T: Clone> {
    pub values: Vec<T>,
}

#[typetag::serde(name = "integerEnum")]
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

#[typetag::serde(name = "floatEnum")]
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

#[typetag::serde(name = "stringEnum")]
impl Constraint for Enumeration<String> {
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
    use crate::core::constraint::common::enumeration::Enumeration;
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

    #[test]
    fn test_enumeration_string() {
        let constraint = Enumeration {
            values: vec!["China".to_string(), "Australia".to_string()],
        };

        let value = Value::String("China".to_string());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("United States".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
