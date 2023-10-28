use crate::core::constraint::Constraint;
use crate::core::field::FieldType;
use crate::core::value::FieldValue;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Type {
    pub typed: FieldType,
}

impl Constraint for Type {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::Boolean(_) if matches!(self.typed, FieldType::Boolean) => Ok(()),
            FieldValue::Integer(_) if matches!(self.typed, FieldType::Integer) => Ok(()),
            FieldValue::UInteger(_) if matches!(self.typed, FieldType::UInteger) => Ok(()),
            FieldValue::Float(_) if matches!(self.typed, FieldType::Float) => Ok(()),
            FieldValue::Email(_) if matches!(self.typed, FieldType::Email) => Ok(()),
            FieldValue::DateTime(_) if matches!(self.typed, FieldType::DateTime) => Ok(()),
            FieldValue::Date(_) if matches!(self.typed, FieldType::Date) => Ok(()),
            FieldValue::Time(_) if matches!(self.typed, FieldType::Time) => Ok(()),
            FieldValue::String(_) if matches!(self.typed, FieldType::String) => Ok(()),
            FieldValue::Array(_) if matches!(self.typed, FieldType::Array) => Ok(()),
            FieldValue::Object(_) if matches!(self.typed, FieldType::Object) => Ok(()),
            _ => Err(anyhow!(format!(
                "value {} is not type {} ({})",
                val, self.typed, "Type"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::common::typed::Type;
    use crate::core::constraint::Constraint;
    use crate::core::field::FieldType;
    use crate::core::value::FieldValue;

    #[test]
    fn validate_boolean() {
        let constraint = Type {
            typed: FieldType::Boolean,
        };

        let value = FieldValue::Boolean(true);
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::String("Test".to_string());
        assert!(constraint.validate(&value).is_err())
    }
}
