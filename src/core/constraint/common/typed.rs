use crate::core::constraint::Constraint;
use crate::core::field::FieldType;
use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDate, NaiveTime};
use email_address_parser::EmailAddress;
use serde_json::Value;

#[derive(Debug)]
pub struct Type {
    pub typed: FieldType,
}

impl Constraint for Type {
    fn validate(&self, val: &Value) -> Result<()> {
        match val {
            Value::Bool(_) if matches!(self.typed, FieldType::Boolean) => Ok(()),
            Value::Number(n) if matches!(self.typed, FieldType::Integer) && n.is_i64() => Ok(()),
            Value::Number(n) if matches!(self.typed, FieldType::Float) && n.is_f64() => Ok(()),
            Value::String(e)
                if matches!(self.typed, FieldType::Email)
                    && EmailAddress::parse(e, None).is_some() =>
            {
                Ok(())
            }
            Value::String(d)
                if matches!(self.typed, FieldType::DateTime)
                    && DateTime::parse_from_rfc3339(d).is_ok() =>
            {
                Ok(())
            }
            Value::String(d)
                if matches!(self.typed, FieldType::Date)
                    && NaiveDate::parse_from_str(d, "%Y-%m-%d").is_ok() =>
            {
                Ok(())
            }
            Value::String(t)
                if matches!(self.typed, FieldType::Time)
                    && NaiveTime::parse_from_str(t, "%H:%M:%S").is_ok() =>
            {
                Ok(())
            }
            Value::String(_) if matches!(self.typed, FieldType::String) => Ok(()),
            Value::Array(_) if matches!(self.typed, FieldType::Array) => Ok(()),
            Value::Object(_) if matches!(self.typed, FieldType::Object) => Ok(()),
            Value::Null => Ok(()),
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
    use serde_json::Value;

    #[test]
    fn validate_boolean() {
        let constraint = Type {
            typed: FieldType::Boolean,
        };

        let value = Value::Bool(true);
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("Test".to_string());
        assert!(constraint.validate(&value).is_err())
    }
}
