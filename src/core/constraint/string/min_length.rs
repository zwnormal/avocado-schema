use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub struct MinLength {
    pub min_length: usize,
}

impl Constraint for MinLength {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::String(v) if v.graphemes(true).count() < self.min_length => {
                Err(anyhow!(format!(
                    "length of {} is less then {} ({})",
                    v, self.min_length, "MinLength"
                )))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::min_length::MinLength;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_min_length() {
        let constraint = MinLength { min_length: 8 };

        let value = Value::String("Valid String".to_string());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Invalid".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
