pub mod date;
pub mod datetime;
pub mod email;
pub mod time;

use crate::core::constraint::string::format::date::validate_date;
use crate::core::constraint::string::format::datetime::validate_datetime;
use crate::core::constraint::string::format::email::validate_email;
use crate::core::constraint::string::format::time::validate_time;
use crate::core::constraint::Constraint;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub enum Format {
    Email,
    Datetime,
    Date,
    Time,
}

impl Serialize for Format {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Format::Email => serializer.serialize_str("email"),
            Format::Datetime => serializer.serialize_str("datetime"),
            Format::Date => serializer.serialize_str("date"),
            Format::Time => serializer.serialize_str("time"),
        }
    }
}

impl<'de> Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FormatVisitor)
    }
}

struct FormatVisitor;

impl<'de> Visitor<'de> for FormatVisitor {
    type Value = Format;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "string field [format] is invalid")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            "email" => Ok(Format::Email),
            "datetime" => Ok(Format::Datetime),
            "date" => Ok(Format::Date),
            "time" => Ok(Format::Time),
            _ => Err(Error::custom("string field [format] is invalid")),
        }
    }
}

impl Constraint for Format {
    fn validate(&self, val: &Value) -> anyhow::Result<()> {
        match self {
            Format::Email => validate_email(val),
            Format::Datetime => validate_datetime(val),
            Format::Date => validate_date(val),
            Format::Time => validate_time(val),
        }
    }
}
