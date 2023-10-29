use crate::core::constraint::Constraint;
use crate::core::value::FieldValue;
use anyhow::{anyhow, Result};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub struct MaxLength {
    pub max_length: usize,
}

impl Constraint for MaxLength {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::String(v) if v.graphemes(true).count() > self.max_length => {
                Err(anyhow!(format!(
                    "length of {} is larger then {} ({})",
                    v,
                    v.graphemes(true).count(),
                    "MaxLength"
                )))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::max_length::MaxLength;
    use crate::core::constraint::Constraint;
    use crate::core::value::FieldValue;

    #[test]
    fn test_max_length() {
        let constraint = MaxLength { max_length: 6 };

        let value = FieldValue::String("Valid".to_string());
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::String("Invalid String".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
