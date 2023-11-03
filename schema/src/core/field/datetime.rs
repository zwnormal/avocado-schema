use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use crate::core::field::{Field, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "datetime")]
pub struct DatetimeField {
    pub name: String
}

impl Field for DatetimeField {
    const FIELD_TYPE: FieldType = FieldType::DateTime;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        vec![Box::new(Type {
            typed: Self::FIELD_TYPE,
        })]
    }
}

#[derive(Default)]
pub struct DatetimeFieldBuilder {
    name: String,
}

impl DatetimeFieldBuilder {
    pub fn new() -> Self {
        DatetimeFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> DatetimeField {
        DatetimeField {
            name: self.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::datetime::{DatetimeField, DatetimeFieldBuilder};
    use crate::visitor::validator::Validator;
    use chrono::Utc;

    #[test]
    fn test_serialize() {
        let field = DatetimeFieldBuilder::new()
            .name("modified")
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"datetime","name":"modified"}"#
        )
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"datetime",
            "name": "modified"
        }"#;
        let field: DatetimeField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "modified");
    }
    #[test]
    fn test_type() {
        let field = DatetimeFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator.validate(&Utc::now()).is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }
}
