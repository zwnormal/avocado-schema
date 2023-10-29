use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use crate::core::field::{Field, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "boolean")]
pub struct BooleanField {
    pub name: String,
    pub title: String,
}

impl Field for BooleanField {
    const FIELD_TYPE: FieldType = FieldType::Boolean;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        vec![Box::new(Type {
            typed: Self::FIELD_TYPE,
        })]
    }
}

#[derive(Default)]
pub struct BooleanFieldBuilder {
    name: String,
    title: String,
}

impl BooleanFieldBuilder {
    pub fn new() -> Self {
        BooleanFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn build(self) -> BooleanField {
        BooleanField {
            name: self.name,
            title: self.title,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::boolean::{BooleanField, BooleanFieldBuilder};
    use crate::visitor::validator::Validator;

    #[test]
    fn test_serialize() {
        let field = BooleanFieldBuilder::new()
            .name("married")
            .title("Married")
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"boolean","name":"married","title":"Married"}"#
        )
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"boolean",
            "name": "married",
            "title": "Married"
        }"#;
        let field: BooleanField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "married");
        assert_eq!(field.title, "Married");
    }

    #[test]
    fn test_type() {
        let field = BooleanFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator.validate(&true).is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }
}
