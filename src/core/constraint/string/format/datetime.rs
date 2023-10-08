use anyhow::anyhow;
use chrono::DateTime;
use serde_json::Value;

pub fn validate_datetime(val: &Value) -> anyhow::Result<()> {
    match val {
        Value::String(v) if DateTime::parse_from_rfc3339(v).is_err() => Err(anyhow!(format!(
            "{} is not a valid RFC 3339 datetime ({})",
            v, "Datetime"
        ))),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::format::datetime::validate_datetime;
    use serde_json::Value;

    #[test]
    fn test_datetime() {
        let value = Value::String("1996-12-19T16:39:57-08:00".to_string());
        assert!(validate_datetime(&value).is_ok());

        let value = Value::String("admin".to_string());
        assert!(validate_datetime(&value).is_err());
    }
}
