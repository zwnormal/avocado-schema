use crate::core::constraint::common::typed::Type;
use crate::core::constraint::integer::enumeration::Enumeration;
use crate::core::constraint::integer::exclusive_maximum::ExclusiveMaximum;
use crate::core::constraint::integer::exclusive_minimum::ExclusiveMinimum;
use crate::core::constraint::integer::maximum::Maximum;
use crate::core::constraint::integer::minimum::Minimum;
use crate::core::constraint::Constraint;
use crate::core::field::FieldEnum;
use crate::core::field::{Field, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename = "integer")]
pub struct IntegerField {
    pub name: String,
    pub title: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<i64>,
}

impl Field for IntegerField {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_type(&self) -> FieldType {
        FieldType::Integer
    }

    fn into_enum(self) -> FieldEnum {
        FieldEnum::Integer(self)
    }

    fn constrains(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(Type {
            typed: FieldType::Integer,
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
pub struct IntegerFieldBuilder {
    name: String,
    title: String,
    enumeration: Option<Vec<i64>>,
    maximum: Option<i64>,
    exclusive_maximum: Option<i64>,
    minimum: Option<i64>,
    exclusive_minimum: Option<i64>,
}

impl IntegerFieldBuilder {
    pub fn new() -> Self {
        IntegerFieldBuilder::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn enumeration(mut self, numbers: Vec<i64>) -> Self {
        self.enumeration = Some(numbers);
        self
    }

    pub fn maximum(mut self, max: i64) -> Self {
        self.maximum = Some(max);
        self
    }

    pub fn exclusive_maximum(mut self, max: i64) -> Self {
        self.exclusive_maximum = Some(max);
        self
    }

    pub fn minimum(mut self, min: i64) -> Self {
        self.minimum = Some(min);
        self
    }

    pub fn exclusive_minimum(mut self, min: i64) -> Self {
        self.exclusive_minimum = Some(min);
        self
    }

    pub fn build(self) -> IntegerField {
        IntegerField {
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