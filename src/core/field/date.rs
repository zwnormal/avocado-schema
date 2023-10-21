use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use crate::core::field::{Field, FieldEnum, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "date")]
pub struct DateField {
    pub name: String,
    pub title: String,
}

impl Field for DateField {
    const FIELD_TYPE: FieldType = FieldType::Date;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn into_enum(self) -> FieldEnum {
        FieldEnum::Date(self)
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        vec![Box::new(Type {
            typed: Self::FIELD_TYPE,
        })]
    }
}

#[derive(Default)]
pub struct DateFieldBuilder {
    name: String,
    title: String,
}

impl DateFieldBuilder {
    pub fn new() -> Self {
        DateFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn build(self) -> DateField {
        DateField {
            name: self.name,
            title: self.title,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::date::{DateField, DateFieldBuilder};
    use crate::visitor::validator::Validator;

    #[test]
    fn test_serialize() {
        let field = DateFieldBuilder::new()
            .name("modified")
            .title("Modified")
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"date","name":"modified","title":"Modified"}"#
        )
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"date",
            "name": "modified",
            "title": "Modified"
        }"#;
        let field: DateField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "modified");
        assert_eq!(field.title, "Modified");
    }
    #[test]
    fn test_type() {
        let field = DateFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator.validate(&"1996-09-19").is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }
}
