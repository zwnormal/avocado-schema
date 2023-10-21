use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde_json::Value;

#[derive(Debug)]
pub struct Minimum<T: Copy> {
    pub min_val: T,
}

impl Constraint for Minimum<i64> {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_i64() && (v.as_i64().unwrap() < self.min_val) => Err(anyhow!(
                format!("value {} is less then {} ({})", v, self.min_val, "Minimum")
            )),
            _ => Ok(()),
        }
    }
}

impl Constraint for Minimum<f64> {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_f64() && (v.as_f64().unwrap() < self.min_val) => Err(anyhow!(
                format!("value {} is less then {} ({})", v, self.min_val, "Minimum")
            )),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::number::minimum::Minimum;
    use crate::core::constraint::Constraint;
    use serde_json::{Number, Value};

    #[test]
    fn test_minimum_i64() {
        let constraint = Minimum { min_val: 10 };

        let value = Value::Number(11.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(10.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_err());
    }

    #[test]
    fn test_minimum_f64() {
        let constraint = Minimum { min_val: 10.0 };

        let value = Value::Number(Number::from_f64(11.0).unwrap());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(10.0).unwrap());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(Number::from_f64(3.0).unwrap());
        assert!(constraint.validate(&value).is_err());
    }
}
