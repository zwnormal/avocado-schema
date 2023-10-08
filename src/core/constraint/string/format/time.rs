use anyhow::anyhow;
use chrono::NaiveTime;
use serde_json::Value;

pub fn validate_time(val: &Value) -> anyhow::Result<()> {
    match val {
        Value::String(v) if NaiveTime::parse_from_str(v, "%H:%M:%S").is_err() => Err(anyhow!(
            format!("{} is not a valid time format of HH:MM:SS ({})", v, "Time")
        )),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::string::format::time::validate_time;
    use serde_json::Value;

    #[test]
    fn test_date() {
        let value = Value::String("23:56:04".to_string());
        assert!(validate_time(&value).is_ok());

        let value = Value::String("admin".to_string());
        assert!(validate_time(&value).is_err());
    }
}
