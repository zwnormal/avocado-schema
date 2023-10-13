use crate::core::constraint::Constraint;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Unique {
    pub unique: bool,
}

#[typetag::serde(name = "unique")]
impl Constraint for Unique {
    fn validate(&self, val: &Value) -> anyhow::Result<()> {
        match val {
            Value::Array(v) if self.unique => {
                let mut values: Vec<Value> = vec![];
                for value in v {
                    if !values.contains(value) {
                        values.push(value.clone())
                    } else {
                        return Err(anyhow!(format!(
                            "array contains duplicated item(s) ({})",
                            "Unique"
                        )));
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::constraint::array::unique::Unique;
    use crate::core::constraint::Constraint;
    use serde_json::{Number, Value};

    #[test]
    fn test_unique() {
        let constraint = Unique { unique: true };

        let value = Value::Array(vec![
            Value::Number(Number::from(1)),
            Value::Number(Number::from(2)),
            Value::Number(Number::from(3)),
        ]);
        assert!(constraint.validate(&value).is_ok());

        let value = Value::Array(vec![
            Value::Number(Number::from(1)),
            Value::Number(Number::from(2)),
            Value::Number(Number::from(2)),
        ]);
        assert!(constraint.validate(&value).is_err());
    }
}
