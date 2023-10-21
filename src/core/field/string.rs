use crate::core::constraint::common::typed::Type;
use crate::core::constraint::string::enumeration::Enumeration;
use crate::core::constraint::string::max_length::MaxLength;
use crate::core::constraint::string::min_length::MinLength;
use crate::core::constraint::string::pattern::Pattern;
use crate::core::constraint::Constraint;
use crate::core::field::FieldEnum;
use crate::core::field::{Field, FieldType};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "string")]
pub struct StringField {
    pub name: String,
    pub title: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<String>>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Pattern>,
}

impl Field for StringField {
    const FIELD_TYPE: FieldType = FieldType::String;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn into_enum(self) -> FieldEnum {
        FieldEnum::String(self)
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: Self::FIELD_TYPE,
        })];
        if let Some(c) = &self.enumeration {
            constraints.push(Box::new(Enumeration { values: c.clone() }))
        }
        if let Some(c) = &self.max_length {
            constraints.push(Box::new(MaxLength { max_length: *c }))
        }
        if let Some(c) = &self.min_length {
            constraints.push(Box::new(MinLength { min_length: *c }))
        }
        if let Some(c) = &self.pattern {
            constraints.push(Box::new(c.clone()))
        }
        constraints
    }
}

#[derive(Default)]
pub struct StringFieldBuilder {
    name: String,
    title: String,
    enumeration: Option<Vec<String>>,
    max_length: Option<usize>,
    min_length: Option<usize>,
    pattern: Option<Regex>,
}

impl StringFieldBuilder {
    pub fn new() -> Self {
        StringFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn enumeration(mut self, strings: Vec<String>) -> Self {
        self.enumeration = Some(strings);
        self
    }

    pub fn max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }

    pub fn min_length(mut self, length: usize) -> Self {
        self.min_length = Some(length);
        self
    }

    pub fn pattern(mut self, pattern: Regex) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn build(self) -> StringField {
        StringField {
            name: self.name,
            title: self.title,
            enumeration: self.enumeration,
            max_length: self.max_length,
            min_length: self.min_length,
            pattern: self.pattern.map(|pattern| Pattern { pattern }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::string::{StringField, StringFieldBuilder};
    use crate::visitor::validator::Validator;
    use regex::Regex;

    #[test]
    fn test_serialize() {
        let field = StringFieldBuilder::new()
            .name("subtype")
            .title("SubType")
            .enumeration(vec!["meeting".to_string(), "email".to_string()])
            .max_length(32)
            .min_length(8)
            .pattern(Regex::new(r"[a-z]+").unwrap())
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"string","name":"subtype","title":"SubType","enum":["meeting","email"],"maxLength":32,"minLength":8,"pattern":"[a-z]+"}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"string",
            "name": "subtype",
            "title": "SubType",
            "enum": ["meeting", "email"],
            "maxLength": 32,
            "minLength": 8,
            "pattern": "[a-z]+"
        }"#;
        let field: StringField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "subtype");
        assert_eq!(field.title, "SubType");
        assert_eq!(field.enumeration.unwrap(), vec!["meeting", "email"]);
        assert_eq!(field.max_length.unwrap(), 32);
        assert_eq!(field.min_length.unwrap(), 8);
        assert_eq!(field.pattern.unwrap().pattern.to_string(), "[a-z]+");
    }

    #[test]
    fn test_type() {
        let field = StringFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator.validate(&"meeting").is_ok());
        assert!(validator.validate(&10).is_err());
    }

    #[test]
    fn test_enumeration() {
        let field = StringFieldBuilder::new()
            .enumeration(vec!["meeting".to_string(), "kickoff".to_string()])
            .build();
        let validator = Validator::new(field);

        assert!(validator.validate(&"meeting").is_ok());
        assert!(validator.validate(&"email").is_err());
    }

    #[test]
    fn test_max_length() {
        let field = StringFieldBuilder::new().max_length(6).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&"email").is_ok());
        assert!(validator.validate(&"emails").is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }

    #[test]
    fn test_min_length() {
        let field = StringFieldBuilder::new().min_length(6).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&"meeting").is_ok());
        assert!(validator.validate(&"emails").is_ok());
        assert!(validator.validate(&"email").is_err());
    }

    #[test]
    fn test_pattern() {
        let field = StringFieldBuilder::new()
            .pattern(Regex::new(r"[a-z]+").unwrap())
            .build();
        let validator = Validator::new(field);

        assert!(validator.validate(&"email").is_ok());
        assert!(validator.validate(&"1234").is_err());
    }
}
