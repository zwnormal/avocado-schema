use anyhow::anyhow;
use chrono::NaiveDate;
use serde_json::Value;

pub fn validate_date(val: &Value) -> anyhow::Result<()> {
    match val {
        Value::String(v) if NaiveDate::parse_from_str(v, "%Y-%m-%d").is_err() => {
            Err(anyhow!(format!(
                "{} is not a valid date format of YYYY-MM-DD ({})",
                v, "Datetime"
            )))
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::format::date::validate_date;
    use serde_json::Value;

    #[test]
    fn test_date() {
        let value = Value::String("1996-09-19".to_string());
        assert!(validate_date(&value).is_ok());

        let value = Value::String("admin".to_string());
        assert!(validate_date(&value).is_err());
    }
}
