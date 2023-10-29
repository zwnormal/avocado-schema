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

macro_rules! field_enum {
    ($($field_name:ident($field:ident)),*) => {
        #[derive(Debug, Deserialize)]
        #[serde(tag = "type", rename_all = "lowercase")]
        pub enum FieldEnum { $(
            $field_name($field),
        )*}

        impl Serialize for FieldEnum {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match &self {
                    $(FieldEnum::$field_name(f) => f.serialize(serializer),)*
                }
            }
        }

        $(
        impl From<$field> for FieldEnum {
            fn from(value: $field) -> Self {
                FieldEnum::$field_name(value)
            }
        }
        )*
    }
}

field_enum!(
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
    Time(TimeField)
);
