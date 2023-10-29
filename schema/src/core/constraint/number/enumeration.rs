use crate::core::constraint::Constraint;
use crate::core::value::FieldValue;
use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct Enumeration<T: Copy> {
    pub values: Vec<T>,
}

impl Constraint for Enumeration<i64> {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::Integer(v) if !self.values.contains(v) => Err(anyhow!(format!(
                "value {} is not valid value ({})",
                v, "Enum"
            ))),
            _ => Ok(()),
        }
    }
}

impl Constraint for Enumeration<u64> {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::UInteger(v) if !self.values.contains(v) => Err(anyhow!(format!(
                "value {} is not valid value ({})",
                v, "Enum"
            ))),
            _ => Ok(()),
        }
    }
}

impl Constraint for Enumeration<f64> {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::Float(v) if !self.values.contains(v) => Err(anyhow!(format!(
                "value {} is not valid value ({})",
                v, "Enum"
            ))),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::number::enumeration::Enumeration;
    use crate::core::constraint::Constraint;
    use crate::core::value::FieldValue;

    #[test]
    fn test_enumeration_i64() {
        let constraint = Enumeration {
            values: vec![1i64, 2i64],
        };

        let value = FieldValue::Integer(1);
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::Integer(3);
        assert!(constraint.validate(&value).is_err());
    }

    #[test]
    fn test_enumeration_u64() {
        let constraint = Enumeration {
            values: vec![1u64, 2u64],
        };

        let value = FieldValue::UInteger(1u64);
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::UInteger(3u64);
        assert!(constraint.validate(&value).is_err());
    }

    #[test]
    fn test_enumeration_f64() {
        let constraint = Enumeration {
            values: vec![1.1, 2.5],
        };

        let value = FieldValue::Float(1.1);
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::Float(3.0);
        assert!(constraint.validate(&value).is_err());
    }
}
