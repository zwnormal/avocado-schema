use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde_json::Value;

#[derive(Debug)]
pub struct Maximum<T: Copy> {
    pub max_val: T,
}

impl Constraint for Maximum<i64> {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_i64() && (v.as_i64().unwrap() > self.max_val) => {
                Err(anyhow!(format!(
                    "value {} is larger then {} ({})",
                    v, self.max_val, "Maximum"
                )))
            }
            _ => Ok(()),
        }
    }
}

impl Constraint for Maximum<f64> {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_f64() && (v.as_f64().unwrap() > self.max_val) => {
                Err(anyhow!(format!(
                    "value {} is larger then {} ({})",
                    v, self.max_val, "Maximum"
                )))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::number::maximum::Maximum;
    use crate::core::constraint::Constraint;
    use serde_json::{Number, Value};

    #[test]
    fn test_maximum_i64() {
        let constraint = Maximum { max_val: 10 };

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(10.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(11.into());
        assert!(constraint.validate(&value).is_err());
    }

    #[test]
    fn test_maximum_f64() {
        let constraint = Maximum { max_val: 10.0 };

        let value = Value::Number(Number::from_f64(3.0).unwrap());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(10.0).unwrap());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(11.0).unwrap());
        assert!(constraint.validate(&value).is_err());
    }
}
