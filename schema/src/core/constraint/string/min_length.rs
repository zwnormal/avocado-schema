use crate::core::constraint::Constraint;
use crate::core::value::FieldValue;
use anyhow::{anyhow, Result};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub struct MinLength {
    pub min_length: usize,
}

impl Constraint for MinLength {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::String(v) if v.graphemes(true).count() < self.min_length => {
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
    use crate::core::value::FieldValue;

    #[test]
    fn test_min_length() {
        let constraint = MinLength { min_length: 8 };

        let value = FieldValue::String("Valid String".to_string());
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::String("Invalid".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
