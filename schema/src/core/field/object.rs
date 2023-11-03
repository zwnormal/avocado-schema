use crate::core::constraint::common::typed::Type;
use crate::core::constraint::object::required::Required;
use crate::core::constraint::Constraint;
use crate::core::field::FieldEnum;
use crate::core::field::{Field, FieldType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "object")]
pub struct ObjectField {
    pub name: String,
    pub properties: HashMap<String, Box<FieldEnum>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl Field for ObjectField {
    const FIELD_TYPE: FieldType = FieldType::Object;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: Self::FIELD_TYPE,
        })];
        if let Some(c) = &self.required {
            constraints.push(Box::new(Required {
                required: c.clone(),
            }))
        }
        constraints
    }
}

#[derive(Default)]
pub struct ObjectFieldBuilder {
    name: String,
    properties: HashMap<String, Box<FieldEnum>>,
    required: Option<Vec<String>>,
}

impl ObjectFieldBuilder {
    pub fn new() -> Self {
        ObjectFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn property(mut self, name: &'static str, field: impl Field) -> Self {
        self.properties
            .insert(name.to_string(), Box::new(field.into()));
        self
    }

    pub fn required(mut self, names: Vec<String>) -> Self {
        self.required = Some(names);
        self
    }

    pub fn build(self) -> ObjectField {
        ObjectField {
            name: self.name,
            properties: self.properties,
            required: self.required,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::array::ArrayFieldBuilder;
    use crate::core::field::boolean::BooleanFieldBuilder;
    use crate::core::field::float::FloatFieldBuilder;
    use crate::core::field::integer::IntegerFieldBuilder;
    use crate::core::field::object::{ObjectField, ObjectFieldBuilder};
    use crate::core::field::string::StringFieldBuilder;
    use crate::core::value::{FieldValue, Reflect};
    use crate::visitor::validator::Validator;
    use std::collections::BTreeMap;

    #[test]
    fn test_serialize() {
        let field = ObjectFieldBuilder::new()
            .name("client")
            .property(
                "first_name",
                StringFieldBuilder::new()
                    .name("first_name")
                    .max_length(32)
                    .min_length(8)
                    .build(),
            )
            .property(
                "last_name",
                StringFieldBuilder::new()
                    .name("last_name")
                    .max_length(32)
                    .min_length(8)
                    .build(),
            )
            .property(
                "age",
                IntegerFieldBuilder::new()
                    .name("age")
                    .maximum(150)
                    .minimum(1)
                    .build(),
            )
            .property(
                "stars",
                FloatFieldBuilder::new()
                    .name("stars")
                    .build(),
            )
            .property(
                "married",
                BooleanFieldBuilder::new()
                    .name("married")
                    .build(),
            )
            .property(
                "categories",
                ArrayFieldBuilder::new()
                    .name("categories")
                    .build(),
            )
            .build();
        serde_json::to_string(&field).unwrap();
    }

    #[test]
    fn test_deserialize() {
        let valid_schema_json = r#"
        {
            "type": "object",
            "name": "client",
            "properties": {
                "first_name": {
                    "type": "string",
                    "name": "first_name",
                    "maxLength": 32,
                    "minLength": 8
                },
                "last_name": {
                    "type": "string",
                    "name": "last_name",
                    "maxLength": 32,
                    "minLength": 8,
                    "pattern": "[a-zA-Z]+"
                }
            }
        }"#;
        assert!(serde_json::from_str::<ObjectField>(valid_schema_json).is_ok());

        let invalid_schema_json = r#"
        {
            "type": "object",
            "name": "client",
            "properties": {
                "first_name": {
                    "type": "string",
                    "name": "first_name",
                    "maxLength": 32,
                    "minLength": -1,
                },
                "last_name": {
                    "type": "string",
                    "name": "last_name",
                    "maxLength": 32,
                    "minLength": 8
                }
            }
        }"#;
        assert!(serde_json::from_str::<ObjectField>(invalid_schema_json).is_err());
    }

    #[test]
    fn test_type() {
        let field = ObjectFieldBuilder::new().build();
        let validator = Validator::new(field);

        struct Client {
            name: Option<String>,
        }

        impl Reflect for Client {
            fn field_value(&self) -> FieldValue {
                FieldValue::Object(BTreeMap::from([(
                    "name".to_string(),
                    self.name.field_value(),
                )]))
            }
        }

        assert!(validator
            .validate(&Client {
                name: Some("Robert Li".to_string())
            })
            .is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }

    #[test]
    fn test_required() {
        let field = ObjectFieldBuilder::new()
            .name("client")
            .property(
                "name",
                StringFieldBuilder::new()
                    .name("name")
                    .max_length(64)
                    .min_length(1)
                    .build(),
            )
            .required(vec!["name".to_string()])
            .build();
        let validator = Validator::new(field);

        struct Client {
            name: Option<String>,
        }

        impl Reflect for Client {
            fn field_value(&self) -> FieldValue {
                FieldValue::Object(BTreeMap::from([(
                    "name".to_string(),
                    self.name.field_value(),
                )]))
            }
        }

        assert!(validator
            .validate(&Client {
                name: Some("Robert Li".to_string())
            })
            .is_ok());
        assert!(validator.validate(&Client { name: None }).is_err());
    }
}
