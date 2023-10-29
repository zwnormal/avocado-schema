use crate::core::constraint::Constraint;
use crate::core::field::array::ArrayField;
use crate::core::field::boolean::BooleanField;
use crate::core::field::date::DateField;
use crate::core::field::datetime::DatetimeField;
use crate::core::field::email::EmailField;
use crate::core::field::float::FloatField;
use crate::core::field::integer::IntegerField;
use crate::core::field::object::ObjectField;
use crate::core::field::string::StringField;
use crate::core::field::time::TimeField;
use crate::core::field::uinteger::UIntegerField;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum FieldType {
    String,
    Integer,
    UInteger,
    Float,
    Boolean,
    Object,
    Array,
    Email,
    DateTime,
    Date,
    Time,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::String => write!(f, "string"),
            FieldType::Integer => write!(f, "integer"),
            FieldType::UInteger => write!(f, "unsigned integer"),
            FieldType::Float => write!(f, "float"),
            FieldType::Boolean => write!(f, "boolean"),
            FieldType::Array => write!(f, "array"),
            FieldType::Object => write!(f, "object"),
            FieldType::Email => write!(f, "email"),
            FieldType::DateTime => write!(f, "datetime"),
            FieldType::Date => write!(f, "date"),
            FieldType::Time => write!(f, "time"),
        }
    }
}

pub trait Field: Debug + Into<FieldEnum> {
    const FIELD_TYPE: FieldType;

    fn name(&self) -> String;
    fn title(&self) -> String;
    fn constrains(&self) -> Vec<Box<dyn Constraint>>;
}

pub mod array;
pub mod boolean;
pub mod date;
pub mod datetime;
pub mod email;
pub mod float;
pub mod integer;
pub mod object;
pub mod string;
pub mod time;
pub mod uinteger;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FieldEnum {
    Array(ArrayField),
    Boolean(BooleanField),
    Float(FloatField),
    Integer(IntegerField),
    UInteger(UIntegerField),
    Object(ObjectField),
    String(StringField),
    Email(EmailField),
    Datetime(DatetimeField),
    Date(DateField),
    Time(TimeField),
}

impl Serialize for FieldEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FieldEnum::Array(f) => f.serialize(serializer),
            FieldEnum::Boolean(f) => f.serialize(serializer),
            FieldEnum::Float(f) => f.serialize(serializer),
            FieldEnum::Integer(f) => f.serialize(serializer),
            FieldEnum::UInteger(f) => f.serialize(serializer),
            FieldEnum::Object(f) => f.serialize(serializer),
            FieldEnum::String(f) => f.serialize(serializer),
            FieldEnum::Email(f) => f.serialize(serializer),
            FieldEnum::Datetime(f) => f.serialize(serializer),
            FieldEnum::Date(f) => f.serialize(serializer),
            FieldEnum::Time(f) => f.serialize(serializer),
        }
    }
}

impl From<ArrayField> for FieldEnum {
    fn from(value: ArrayField) -> Self {
        FieldEnum::Array(value)
    }
}

impl From<BooleanField> for FieldEnum {
    fn from(value: BooleanField) -> Self {
        FieldEnum::Boolean(value)
    }
}

impl From<DateField> for FieldEnum {
    fn from(value: DateField) -> Self {
        FieldEnum::Date(value)
    }
}

impl From<DatetimeField> for FieldEnum {
    fn from(value: DatetimeField) -> Self {
        FieldEnum::Datetime(value)
    }
}

impl From<EmailField> for FieldEnum {
    fn from(value: EmailField) -> Self {
        FieldEnum::Email(value)
    }
}

impl From<FloatField> for FieldEnum {
    fn from(value: FloatField) -> Self {
        FieldEnum::Float(value)
    }
}

impl From<IntegerField> for FieldEnum {
    fn from(value: IntegerField) -> Self {
        FieldEnum::Integer(value)
    }
}

impl From<UIntegerField> for FieldEnum {
    fn from(value: UIntegerField) -> Self {
        FieldEnum::UInteger(value)
    }
}

impl From<ObjectField> for FieldEnum {
    fn from(value: ObjectField) -> Self {
        FieldEnum::Object(value)
    }
}

impl From<StringField> for FieldEnum {
    fn from(value: StringField) -> Self {
        FieldEnum::String(value)
    }
}

impl From<TimeField> for FieldEnum {
    fn from(value: TimeField) -> Self {
        FieldEnum::Time(value)
    }
}
