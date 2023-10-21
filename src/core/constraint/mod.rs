use anyhow::Result;
use serde_json::Value;

pub trait Constraint {
    fn validate(&self, val: &Value) -> Result<()>;
}

pub mod array;
pub mod common;
pub mod number;
pub mod object;
pub mod string;
