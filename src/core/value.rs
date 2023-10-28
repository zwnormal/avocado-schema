use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};
use email_address_parser::EmailAddress;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    String(String),
    Integer(i64),
    UInteger(u64),
    Float(f64),
    Boolean(bool),
    Object(BTreeMap<String, FieldValue>),
    Array(Vec<FieldValue>),
    Email(EmailAddress),
    DateTime(DateTime<Utc>),
    Date(NaiveDate),
    Time(NaiveTime),
    Null,
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldValue::String(v) => write!(f, "{}", v),
            FieldValue::Integer(v) => write!(f, "{}", v),
            FieldValue::UInteger(v) => write!(f, "{}", v),
            FieldValue::Float(v) => write!(f, "{}", v),
            FieldValue::Boolean(v) => write!(f, "{}", v),
            FieldValue::Object(_) => write!(f, "Object(...)"),
            FieldValue::Array(_) => write!(f, "Array(...)"),
            FieldValue::Email(v) => write!(f, "{}", v),
            FieldValue::DateTime(v) => write!(f, "{}", v),
            FieldValue::Date(v) => write!(f, "{}", v),
            FieldValue::Time(v) => write!(f, "{}", v),
            FieldValue::Null => write!(f, "null"),
        }
    }
}

impl From<i8> for FieldValue {
    fn from(value: i8) -> Self {
        FieldValue::Integer(value as i64)
    }
}

impl From<i16> for FieldValue {
    fn from(value: i16) -> Self {
        FieldValue::Integer(value as i64)
    }
}

impl From<i32> for FieldValue {
    fn from(value: i32) -> Self {
        FieldValue::Integer(value as i64)
    }
}

impl From<i64> for FieldValue {
    fn from(value: i64) -> Self {
        FieldValue::Integer(value)
    }
}

impl From<u8> for FieldValue {
    fn from(value: u8) -> Self {
        FieldValue::UInteger(value as u64)
    }
}

impl From<u16> for FieldValue {
    fn from(value: u16) -> Self {
        FieldValue::UInteger(value as u64)
    }
}

impl From<u32> for FieldValue {
    fn from(value: u32) -> Self {
        FieldValue::UInteger(value as u64)
    }
}

impl From<u64> for FieldValue {
    fn from(value: u64) -> Self {
        FieldValue::UInteger(value)
    }
}

impl From<f32> for FieldValue {
    fn from(value: f32) -> Self {
        FieldValue::Float(value as f64)
    }
}

impl From<f64> for FieldValue {
    fn from(value: f64) -> Self {
        FieldValue::Float(value)
    }
}

impl From<bool> for FieldValue {
    fn from(value: bool) -> Self {
        FieldValue::Boolean(value)
    }
}

impl From<&'static str> for FieldValue {
    fn from(value: &'static str) -> Self {
        FieldValue::String(value.to_string())
    }
}

impl From<String> for FieldValue {
    fn from(value: String) -> Self {
        FieldValue::String(value)
    }
}

impl From<EmailAddress> for FieldValue {
    fn from(value: EmailAddress) -> Self {
        FieldValue::Email(value)
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for FieldValue {
    fn from(value: DateTime<Tz>) -> Self {
        FieldValue::DateTime(value.with_timezone(&Utc))
    }
}

impl From<NaiveDate> for FieldValue {
    fn from(value: NaiveDate) -> Self {
        FieldValue::Date(value)
    }
}

impl From<NaiveTime> for FieldValue {
    fn from(value: NaiveTime) -> Self {
        FieldValue::Time(value)
    }
}

impl<T: Into<FieldValue>> From<Option<T>> for FieldValue {
    fn from(value: Option<T>) -> Self {
        match value {
            None => FieldValue::Null,
            Some(v) => v.into(),
        }
    }
}

impl<T: Into<FieldValue>> From<Vec<T>> for FieldValue {
    fn from(value: Vec<T>) -> Self {
        FieldValue::Array(value.into_iter().map(|v| v.into()).collect())
    }
}
