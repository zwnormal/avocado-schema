use crate::base::field::{Field, FieldType};
use crate::base::visitor::FieldEnum;
use crate::core::constraint::array::unique::Unique;
use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
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
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_type(&self) -> FieldType {
        FieldType::Array
    }

    fn into_enum(self) -> FieldEnum {
        FieldEnum::Array(self)
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: FieldType::Array,
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
        self.item = Some(item.into_enum());
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
            item: self.item.map(|item| Box::new(item)),
            unique: self.unique,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::array::ArrayFieldBuilder;
    use crate::core::string::StringFieldBuilder;

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
}
