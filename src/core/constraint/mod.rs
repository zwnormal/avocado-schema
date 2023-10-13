use anyhow::Result;
use serde_json::Value;
use std::fmt::Debug;

#[typetag::serde(tag = "constraint")]
pub trait Constraint: Debug {
    fn validate(&self, val: &Value) -> Result<()>;
}

pub mod array;
pub mod common;
pub mod number;
pub mod object;
pub mod string;
