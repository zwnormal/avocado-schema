use crate::core::constraint::common::typed::Type;
use crate::core::constraint::number::enumeration::Enumeration;
use crate::core::constraint::number::exclusive_maximum::ExclusiveMaximum;
use crate::core::constraint::number::exclusive_minimum::ExclusiveMinimum;
use crate::core::constraint::number::maximum::Maximum;
use crate::core::constraint::number::minimum::Minimum;
use crate::core::constraint::Constraint;
use crate::core::field::{Field, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "uinteger")]
pub struct UIntegerField {
    pub name: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<u64>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<u64>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<u64>,
}

impl Field for UIntegerField {
    const FIELD_TYPE: FieldType = FieldType::UInteger;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: Self::FIELD_TYPE,
        })];
        if let Some(c) = &self.enumeration {
            constraints.push(Box::new(Enumeration { values: c.clone() }))
        }
        if let Some(c) = &self.maximum {
            constraints.push(Box::new(Maximum { max_val: *c }))
        }
        if let Some(c) = &self.exclusive_maximum {
            constraints.push(Box::new(ExclusiveMaximum { max_val: *c }))
        }
        if let Some(c) = &self.minimum {
            constraints.push(Box::new(Minimum { min_val: *c }))
        }
        if let Some(c) = &self.exclusive_minimum {
            constraints.push(Box::new(ExclusiveMinimum { min_val: *c }))
        }
        constraints
    }
}

#[derive(Default)]
pub struct UIntegerFieldBuilder {
    name: String,
    enumeration: Option<Vec<u64>>,
    maximum: Option<u64>,
    exclusive_maximum: Option<u64>,
    minimum: Option<u64>,
    exclusive_minimum: Option<u64>,
}

impl UIntegerFieldBuilder {
    pub fn new() -> Self {
        UIntegerFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn enumeration(mut self, numbers: Vec<u64>) -> Self {
        self.enumeration = Some(numbers);
        self
    }

    pub fn maximum(mut self, max: u64) -> Self {
        self.maximum = Some(max);
        self
    }

    pub fn exclusive_maximum(mut self, max: u64) -> Self {
        self.exclusive_maximum = Some(max);
        self
    }

    pub fn minimum(mut self, min: u64) -> Self {
        self.minimum = Some(min);
        self
    }

    pub fn exclusive_minimum(mut self, min: u64) -> Self {
        self.exclusive_minimum = Some(min);
        self
    }

    pub fn build(self) -> UIntegerField {
        UIntegerField {
            name: self.name,
            enumeration: self.enumeration,
            maximum: self.maximum,
            exclusive_maximum: self.exclusive_maximum,
            minimum: self.minimum,
            exclusive_minimum: self.exclusive_minimum,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::uinteger::{UIntegerField, UIntegerFieldBuilder};
    use crate::visitor::validator::Validator;

    #[test]
    fn test_serialize() {
        let field = UIntegerFieldBuilder::new()
            .name("age")
            .enumeration(vec![50, 100])
            .maximum(100)
            .exclusive_maximum(101)
            .minimum(1)
            .exclusive_minimum(0)
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"uinteger","name":"age","enum":[50,100],"maximum":100,"exclusiveMaximum":101,"minimum":1,"exclusiveMinimum":0}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"uinteger",
            "name": "age",
            "enum": [50, 100],
            "maximum": 100,
            "exclusiveMaximum": 101,
            "minimum": 1,
            "exclusiveMinimum": 0
        }"#;
        let field: UIntegerField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "age");
        assert_eq!(field.enumeration.unwrap(), vec![50, 100]);
        assert_eq!(field.maximum.unwrap(), 100);
        assert_eq!(field.exclusive_maximum.unwrap(), 101);
        assert_eq!(field.minimum.unwrap(), 1);
        assert_eq!(field.exclusive_minimum.unwrap(), 0);
    }

    #[test]
    fn test_type() {
        let field = UIntegerFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator.validate(&10u64).is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }

    #[test]
    fn test_enumeration() {
        let field = UIntegerFieldBuilder::new()
            .enumeration(vec![50, 100])
            .build();
        let validator = Validator::new(field);

        assert!(validator.validate(&50u64).is_ok());
        assert!(validator.validate(&30u64).is_err());
    }

    #[test]
    fn test_maximum() {
        let field = UIntegerFieldBuilder::new().maximum(100).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&99u64).is_ok());
        assert!(validator.validate(&100u64).is_ok());
        assert!(validator.validate(&101u64).is_err());
    }

    #[test]
    fn test_exclusive_maximum() {
        let field = UIntegerFieldBuilder::new().exclusive_maximum(100).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&99u64).is_ok());
        assert!(validator.validate(&100u64).is_err());
        assert!(validator.validate(&101u64).is_err());
    }

    #[test]
    fn test_minimum() {
        let field = UIntegerFieldBuilder::new().minimum(1).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&2u64).is_ok());
        assert!(validator.validate(&1u64).is_ok());
        assert!(validator.validate(&0u64).is_err())
    }

    #[test]
    fn test_exclusive_minimum() {
        let field = UIntegerFieldBuilder::new().exclusive_minimum(1).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&2u64).is_ok());
        assert!(validator.validate(&1u64).is_err());
        assert!(validator.validate(&0u64).is_err())
    }
}
