use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use crate::core::field::{Field, FieldEnum, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "datetime")]
pub struct DatetimeField {
    pub name: String,
    pub title: String,
}

impl Field for DatetimeField {
    const FIELD_TYPE: FieldType = FieldType::DateTime;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn into_enum(self) -> FieldEnum {
        FieldEnum::Datetime(self)
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
    title: String,
}

impl DatetimeFieldBuilder {
    pub fn new() -> Self {
        DatetimeFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn build(self) -> DatetimeField {
        DatetimeField {
            name: self.name,
            title: self.title,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::datetime::{DatetimeField, DatetimeFieldBuilder};
    use crate::visitor::validator::Validator;

    #[test]
    fn test_serialize() {
        let field = DatetimeFieldBuilder::new()
            .name("modified")
            .title("Modified")
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"datetime","name":"modified","title":"Modified"}"#
        )
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"datetime",
            "name": "modified",
            "title": "Modified"
        }"#;
        let field: DatetimeField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "modified");
        assert_eq!(field.title, "Modified");
    }
    #[test]
    fn test_type() {
        let field = DatetimeFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator.validate(&"1996-12-19T16:39:57-08:00").is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }
}
