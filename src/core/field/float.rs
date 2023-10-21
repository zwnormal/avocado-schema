use crate::core::constraint::common::typed::Type;
use crate::core::constraint::number::enumeration::Enumeration;
use crate::core::constraint::number::exclusive_maximum::ExclusiveMaximum;
use crate::core::constraint::number::exclusive_minimum::ExclusiveMinimum;
use crate::core::constraint::number::maximum::Maximum;
use crate::core::constraint::number::minimum::Minimum;
use crate::core::constraint::Constraint;
use crate::core::field::FieldEnum;
use crate::core::field::{Field, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "float")]
pub struct FloatField {
    pub name: String,
    pub title: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<f64>,
}

impl Field for FloatField {
    const FIELD_TYPE: FieldType = FieldType::Float;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn into_enum(self) -> FieldEnum {
        FieldEnum::Float(self)
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
pub struct FloatFieldBuilder {
    name: String,
    title: String,
    enumeration: Option<Vec<f64>>,
    maximum: Option<f64>,
    exclusive_maximum: Option<f64>,
    minimum: Option<f64>,
    exclusive_minimum: Option<f64>,
}

impl FloatFieldBuilder {
    pub fn new() -> Self {
        FloatFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn enumeration(mut self, numbers: Vec<f64>) -> Self {
        self.enumeration = Some(numbers);
        self
    }

    pub fn maximum(mut self, max: f64) -> Self {
        self.maximum = Some(max);
        self
    }

    pub fn exclusive_maximum(mut self, max: f64) -> Self {
        self.exclusive_maximum = Some(max);
        self
    }

    pub fn minimum(mut self, min: f64) -> Self {
        self.minimum = Some(min);
        self
    }

    pub fn exclusive_minimum(mut self, min: f64) -> Self {
        self.exclusive_minimum = Some(min);
        self
    }

    pub fn build(self) -> FloatField {
        FloatField {
            name: self.name,
            title: self.title,
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
    use crate::core::field::float::{FloatField, FloatFieldBuilder};
    use crate::visitor::validator::Validator;

    #[test]
    fn test_serialize() {
        let field = FloatFieldBuilder::new()
            .name("price")
            .title("Price")
            .enumeration(vec![10.0, 20.0])
            .maximum(20.0)
            .exclusive_maximum(20.1)
            .minimum(10.0)
            .exclusive_minimum(9.9)
            .build();
        let field_json = serde_json::to_string(&field).unwrap();
        assert_eq!(
            field_json,
            r#"{"type":"float","name":"price","title":"Price","enum":[10.0,20.0],"maximum":20.0,"exclusiveMaximum":20.1,"minimum":10.0,"exclusiveMinimum":9.9}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let field_json = r#"
        {
            "type":"float",
            "name": "price",
            "title": "Price",
            "enum": [10.0, 20.0],
            "maximum": 20.0,
            "exclusiveMaximum": 20.1,
            "minimum": 10.0,
            "exclusiveMinimum": 9.9
        }"#;
        let field: FloatField = serde_json::from_str(field_json).unwrap();
        assert_eq!(field.name, "price");
        assert_eq!(field.title, "Price");
        assert_eq!(field.enumeration.unwrap(), vec![10.0, 20.0]);
        assert_eq!(field.maximum.unwrap(), 20.0);
        assert_eq!(field.exclusive_maximum.unwrap(), 20.1);
        assert_eq!(field.minimum.unwrap(), 10.0);
        assert_eq!(field.exclusive_minimum.unwrap(), 9.9);
    }

    #[test]
    fn test_type() {
        let field = FloatFieldBuilder::new().build();
        let validator = Validator::new(field);

        assert!(validator.validate(&10.0).is_ok());
        assert!(validator.validate(&"meeting").is_err());
    }

    #[test]
    fn test_enumeration() {
        let field = FloatFieldBuilder::new()
            .enumeration(vec![10.0, 20.0])
            .build();
        let validator = Validator::new(field);

        assert!(validator.validate(&10.0).is_ok());
        assert!(validator.validate(&20.1).is_err());
    }

    #[test]
    fn test_maximum() {
        let field = FloatFieldBuilder::new().maximum(20.0).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&19.9).is_ok());
        assert!(validator.validate(&20.0).is_ok());
        assert!(validator.validate(&20.1).is_err());
    }

    #[test]
    fn test_exclusive_maximum() {
        let field = FloatFieldBuilder::new().exclusive_maximum(20.0).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&19.9).is_ok());
        assert!(validator.validate(&20.0).is_err());
        assert!(validator.validate(&20.1).is_err());
    }

    #[test]
    fn test_minimum() {
        let field = FloatFieldBuilder::new().minimum(10.0).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&10.1).is_ok());
        assert!(validator.validate(&10.0).is_ok());
        assert!(validator.validate(&9.9).is_err());
    }

    #[test]
    fn test_exclusive_minimum() {
        let field = FloatFieldBuilder::new().exclusive_minimum(10.0).build();
        let validator = Validator::new(field);

        assert!(validator.validate(&10.1).is_ok());
        assert!(validator.validate(&10.0).is_err());
        assert!(validator.validate(&9.9).is_err());
    }
}
