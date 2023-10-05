use crate::core::constraint::Constraint;
use anyhow::anyhow;
use email_address_parser::EmailAddress;
use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct Email;

impl Constraint for Email {
    fn validate(&self, val: &Value) -> anyhow::Result<()> {
        match val {
            Value::String(v) if EmailAddress::parse(v, None).is_none() => Err(anyhow!(format!(
                "{} is not a valid email address ({})",
                v, "Email"
            ))),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::format::email::Email;
    use crate::core::constraint::Constraint;
    use serde_json::Value;

    #[test]
    fn test_email() {
        let constraint = Email;
        let value = Value::String("admin@avocado.com".to_string());
        assert!(constraint.validate(&value).is_ok());

        let value = Value::String("admin".to_string());
        assert!(constraint.validate(&value).is_err());
    }
}
