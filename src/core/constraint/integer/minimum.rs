use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Minimum {
    pub min_val: i64,
}

impl Constraint for Minimum {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Number(v) if v.is_i64() && (v.as_i64().unwrap() < self.min_val) => Err(anyhow!(
                format!("value {} is less then {} ({})", v, self.min_val, "Minimum")
            )),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::integer::minimum::Minimum;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_minimum() {
        let constraint = Minimum { min_val: 10 };

        let value = Value::Number(11.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(10.into());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Number(3.into());
        assert!(constraint.validate(&value).is_err());
    }
}
