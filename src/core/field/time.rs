use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use crate::core::field::{Field, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "time")]
pub struct TimeField {
    pub name: String,
    pub title: String,
}

impl Field for TimeField {
    const FIELD_TYPE: FieldType = FieldType::Time;

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
pub struct TimeFieldBuilder {
    name: String,
    title: String,
}

impl TimeFieldBuilder {
    pub fn new() -> Self {
        TimeFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn build(self) -> TimeField {
        TimeField {
            name: self.name,
            title: self.title,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::time::{TimeField, TimeFieldBuilder};
    use crate::visitor::validator::Validator;
    use chrono::NaiveTime;

    #[test]
    fn test_serialize() {
        let field = TimeFieldBuilder::new()
            .name("modified")
            .title("Modified")
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"time","name":"modified","title":"Modified"}"#
        )
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"time",
            "name": "modified",
            "title": "Modified"
        }"#;
        let field: TimeField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "modified");
        assert_eq!(field.title, "Modified");
    }
    #[test]
    fn test_type() {
        let field = TimeFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator
            .validate(&NaiveTime::from_hms_opt(23, 56, 4))
            .is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }
}
