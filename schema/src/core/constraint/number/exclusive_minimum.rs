use crate::core::constraint::Constraint;
use crate::core::value::FieldValue;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct ExclusiveMinimum<T: Copy> {
    pub min_val: T,
}

impl Constraint for ExclusiveMinimum<i64> {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::Integer(v) if (*v <= self.min_val) => Err(anyhow!(format!(
                "value {} is less then or equals to {} ({})",
                v, self.min_val, "ExclusiveMinimum"
            ))),
            _ => Ok(()),
        }
    }
}

impl Constraint for ExclusiveMinimum<u64> {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::UInteger(v) if (*v <= self.min_val) => Err(anyhow!(format!(
                "value {} is less then or equals to {} ({})",
                v, self.min_val, "ExclusiveMinimum"
            ))),
            _ => Ok(()),
        }
    }
}

impl Constraint for ExclusiveMinimum<f64> {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::Float(v) if (*v <= self.min_val) => Err(anyhow!(format!(
                "value {} is less then or equals to {} ({})",
                v, self.min_val, "ExclusiveMinimum"
            ))),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::number::exclusive_minimum::ExclusiveMinimum;
    use crate::core::constraint::Constraint;
    use crate::core::value::FieldValue;

    #[test]
    fn test_exclusive_minimum_i64() {
        let constraint = ExclusiveMinimum { min_val: 10i64 };

        let value = FieldValue::Integer(11);
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::Integer(10);
        assert!(constraint.validate(&value).is_err());

        let value = FieldValue::Integer(3);
        assert!(constraint.validate(&value).is_err());
    }

    #[test]
    fn test_exclusive_minimum_u64() {
        let constraint = ExclusiveMinimum { min_val: 10u64 };

        let value = FieldValue::UInteger(11u64);
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::UInteger(10u64);
        assert!(constraint.validate(&value).is_err());

        let value = FieldValue::UInteger(3u64);
        assert!(constraint.validate(&value).is_err());
    }

    #[test]
    fn test_exclusive_minimum_f64() {
        let constraint = ExclusiveMinimum { min_val: 10.0 };

        let value = FieldValue::Float(11.0);
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::Float(10.0);
        assert!(constraint.validate(&value).is_err());

        let value = FieldValue::Float(3.0);
        assert!(constraint.validate(&value).is_err());
    }
}
