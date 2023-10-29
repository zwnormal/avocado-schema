use crate::core::constraint::Constraint;
use crate::core::value::FieldValue;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Unique {
    pub unique: bool,
}

impl Constraint for Unique {
    fn validate(&self, val: &FieldValue) -> anyhow::Result<()> {
        match val {
            FieldValue::Array(v) if self.unique => {
                let mut values: Vec<FieldValue> = vec![];
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
    use crate::core::value::FieldValue;

    #[test]
    fn test_unique() {
        let constraint = Unique { unique: true };

        let value = FieldValue::Array(vec![
            FieldValue::Integer(1),
            FieldValue::Integer(2),
            FieldValue::Integer(3),
        ]);
        assert!(constraint.validate(&value).is_ok());

        let value = FieldValue::Array(vec![
            FieldValue::Integer(1),
            FieldValue::Integer(2),
            FieldValue::Integer(2),
        ]);
        assert!(constraint.validate(&value).is_err());
    }
}
