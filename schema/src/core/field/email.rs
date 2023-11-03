use crate::core::constraint::common::typed::Type;
use crate::core::constraint::Constraint;
use crate::core::field::{Field, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "email")]
pub struct EmailField {
    pub name: String,
}

impl Field for EmailField {
    const FIELD_TYPE: FieldType = FieldType::Email;

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
pub struct EmailFieldBuilder {
    name: String,
}

impl EmailFieldBuilder {
    pub fn new() -> Self {
        EmailFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> EmailField {
        EmailField { name: self.name }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::email::{EmailField, EmailFieldBuilder};
    use crate::visitor::validator::Validator;
    use email_address_parser::EmailAddress;

    #[test]
    fn test_serialize() {
        let field = EmailFieldBuilder::new().name("email").build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(field_json, r#"{"type":"email","name":"email"}"#)
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"email",
            "name": "email"
        }"#;
        let field: EmailField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "email");
    }

    #[test]
    fn test_type() {
        let field = EmailFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator
            .validate(&EmailAddress::parse("admin@avocado.com", None))
            .is_ok());
        assert!(validator
            .validate(&EmailAddress::parse("meeting", None))
            .is_err());
    }
}
