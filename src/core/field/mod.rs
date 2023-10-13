use crate::core::constraint::Constraint;
use crate::core::field::array::ArrayField;
use crate::core::field::boolean::BooleanField;
use crate::core::field::float::FloatField;
use crate::core::field::integer::IntegerField;
use crate::core::field::object::ObjectField;
use crate::core::field::string::StringField;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    Object,
    Array,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::String => write!(f, "string"),
            FieldType::Integer => write!(f, "integer"),
            FieldType::Float => write!(f, "float"),
            FieldType::Boolean => write!(f, "boolean"),
            FieldType::Array => write!(f, "array"),
            FieldType::Object => write!(f, "object"),
        }
    }
}

pub trait Field: Debug {
    const FIELD_TYPE: FieldType;

    fn name(&self) -> String;
    fn title(&self) -> String;
    fn into_enum(self) -> FieldEnum;
    fn constrains(&self) -> Vec<Arc<dyn Constraint>>;
}

pub mod array;
pub mod boolean;
pub mod float;
pub mod integer;
pub mod object;
pub mod string;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FieldEnum {
    Array(ArrayField),
    Boolean(BooleanField),
    Float(FloatField),
    Integer(IntegerField),
    Object(ObjectField),
    String(StringField),
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
            FieldEnum::Object(f) => f.serialize(serializer),
            FieldEnum::String(f) => f.serialize(serializer),
        }
    }
}
