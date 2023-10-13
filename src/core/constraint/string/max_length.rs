use crate::core::constraint::Constraint;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaxLength {
    pub max_length: usize,
}

#[typetag::serde(name = "maxLength")]
impl Constraint for MaxLength {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::String(v) if v.graphemes(true).count() > self.max_length => {
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
    use serde_json::Value;

    #[test]
    fn test_max_length() {
        let constraint = MaxLength { max_length: 6 };

        let value = Value::String("Valid".to_string());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Invalid String".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
