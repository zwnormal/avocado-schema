use anyhow::anyhow;
use email_address_parser::EmailAddress;
use serde_json::Value;

pub fn validate_email(val: &Value) -> anyhow::Result<()> {
    match val {
        Value::String(v) if EmailAddress::parse(v, None).is_none() => Err(anyhow!(format!(
            "{} is not a valid email address ({})",
            v, "Email"
        ))),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::format::email::validate_email;
    use serde_json::Value;

    #[test]
    fn test_email() {
        let value = Value::String("admin@avocado.com".to_string());
        assert!(validate_email(&value).is_ok());

        let value = Value::String("admin".to_string());
        assert!(validate_email(&value).is_err());
    }
}
