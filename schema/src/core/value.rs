use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};
use email_address_parser::EmailAddress;
use secrecy::{ExposeSecret, Secret, Zeroize};
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

pub trait Reflect {
    fn field_value(&self) -> FieldValue;
}

impl Reflect for i8 {
    fn field_value(&self) -> FieldValue {
        FieldValue::Integer(*self as i64)
    }
}

impl Reflect for i16 {
    fn field_value(&self) -> FieldValue {
        FieldValue::Integer(*self as i64)
    }
}

impl Reflect for i32 {
    fn field_value(&self) -> FieldValue {
        FieldValue::Integer(*self as i64)
    }
}

impl Reflect for i64 {
    fn field_value(&self) -> FieldValue {
        FieldValue::Integer(*self)
    }
}

impl Reflect for u8 {
    fn field_value(&self) -> FieldValue {
        FieldValue::UInteger(*self as u64)
    }
}

impl Reflect for u16 {
    fn field_value(&self) -> FieldValue {
        FieldValue::UInteger(*self as u64)
    }
}

impl Reflect for u32 {
    fn field_value(&self) -> FieldValue {
        FieldValue::UInteger(*self as u64)
    }
}

impl Reflect for u64 {
    fn field_value(&self) -> FieldValue {
        FieldValue::UInteger(*self)
    }
}

impl Reflect for f32 {
    fn field_value(&self) -> FieldValue {
        FieldValue::Float(*self as f64)
    }
}

impl Reflect for f64 {
    fn field_value(&self) -> FieldValue {
        FieldValue::Float(*self)
    }
}

impl Reflect for bool {
    fn field_value(&self) -> FieldValue {
        FieldValue::Boolean(*self)
    }
}

impl Reflect for &'static str {
    fn field_value(&self) -> FieldValue {
        FieldValue::String(self.to_string())
    }
}

impl Reflect for String {
    fn field_value(&self) -> FieldValue {
        FieldValue::String(self.clone())
    }
}

impl Reflect for EmailAddress {
    fn field_value(&self) -> FieldValue {
        FieldValue::Email(self.clone())
    }
}

impl<Tz: TimeZone> Reflect for DateTime<Tz> {
    fn field_value(&self) -> FieldValue {
        FieldValue::DateTime(self.with_timezone(&Utc))
    }
}

impl Reflect for NaiveDate {
    fn field_value(&self) -> FieldValue {
        FieldValue::Date(self.clone())
    }
}

impl Reflect for NaiveTime {
    fn field_value(&self) -> FieldValue {
        FieldValue::Time(self.clone())
    }
}

impl<T: Reflect + Zeroize> Reflect for Secret<T> {
    fn field_value(&self) -> FieldValue {
        self.expose_secret().field_value()
    }
}

impl<T: Reflect> Reflect for Option<T> {
    fn field_value(&self) -> FieldValue {
        match self {
            None => FieldValue::Null,
            Some(v) => v.field_value(),
        }
    }
}

impl<T: Reflect> Reflect for Vec<T> {
    fn field_value(&self) -> FieldValue {
        FieldValue::Array(self.into_iter().map(|v| v.field_value()).collect())
    }
}
