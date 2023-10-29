use crate::core::value::FieldValue;
use anyhow::Result;

pub trait Constraint {
    fn validate(&self, val: &FieldValue) -> Result<()>;
}

pub mod array;
pub mod common;
pub mod number;
pub mod object;
pub mod string;
