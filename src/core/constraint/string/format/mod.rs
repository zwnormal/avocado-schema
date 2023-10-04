use crate::core::constraint::string::format::email::Email;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

pub mod email;

#[derive(Debug, PartialEq)]
pub enum Format {
    Email(Email),
}

impl Serialize for Format {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Format::Email(_) => serializer.serialize_str("email"),
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
            "email" => Ok(Format::Email(Email)),
            _ => Err(Error::custom("string field [format] is invalid")),
        }
    }
}
