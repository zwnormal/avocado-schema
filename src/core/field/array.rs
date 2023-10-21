use crate::core::constraint::array::unique::Unique;
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use crate::core::field::FieldEnum;
use crate::core::field::{Field, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "array")]
pub struct ArrayField {
    pub name: String,
    pub title: String,
    pub item: Option<Box<FieldEnum>>,
    pub unique: Option<bool>,
}

impl Field for ArrayField {
    const FIELD_TYPE: FieldType = FieldType::Array;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: Self::FIELD_TYPE,
        })];
        if let Some(c) = self.unique {
            constraints.push(Box::new(Unique { unique: c }));
        }
        constraints
    }
}

#[derive(Default)]
pub struct ArrayFieldBuilder {
    name: String,
    title: String,
    item: Option<FieldEnum>,
    unique: Option<bool>,
}

impl ArrayFieldBuilder {
    pub fn new() -> ArrayFieldBuilder {
        ArrayFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn item(mut self, item: impl Field) -> Self {
        self.item = Some(item.into());
        self
    }

    pub fn unique(mut self, unique: bool) -> Self {
        self.unique = Some(unique);
        self
    }

    pub fn build(self) -> ArrayField {
        ArrayField {
            name: self.name,
            title: self.title,
            item: self.item.map(Box::new),
            unique: self.unique,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::array::{ArrayField, ArrayFieldBuilder};
    use crate::core::field::string::StringFieldBuilder;
    use crate::core::field::FieldEnum;
    use crate::visitor::validator::Validator;

    #[test]
    fn test_serialize() {
        let field = ArrayFieldBuilder::new()
            .name("tags")
            .title("Tags")
            .item(StringFieldBuilder::new().build())
            .unique(true)
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"array","name":"tags","title":"Tags","item":{"type":"string","name":"","title":""},"unique":true}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"array",
            "name": "tags",
            "title": "Tags",
            "item": {
                "type":"string",
                "name":"tag",
                "title":"Tag"
            },
            "unique": true
        }"#;
        let field: ArrayField = serde_json::from_str(field_json).unwrap();
        assert!(matches!(*field.item.unwrap(), FieldEnum::String(_)));
        assert!(field.unique.unwrap());
    }

    #[test]
    fn test_type() {
        let field = ArrayFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator.validate(&vec![10, 20]).is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }

    #[test]
    fn test_item() {
        let field = ArrayFieldBuilder::new()
            .item(StringFieldBuilder::new().build())
            .build();
        let validator = Validator::new(field);

        assert!(validator.validate(&vec!["meeting", "email"]).is_ok());
        assert!(validator.validate(&vec![1, 2]).is_err());
    }

    #[test]
    fn test_unique() {
        let field = ArrayFieldBuilder::new().unique(true).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&vec![1, 2, 3]).is_ok());
        assert!(validator.validate(&vec![1, 2, 2]).is_err());
    }
}
