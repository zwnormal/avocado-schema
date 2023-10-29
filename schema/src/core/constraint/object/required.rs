use crate::core::constraint::Constraint;
use crate::core::value::FieldValue;
use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct Required {
    pub required: Vec<String>,
}

impl Constraint for Required {
    fn validate(&self, val: &FieldValue) -> Result<()> {
        match val {
            FieldValue::Object(o) => {
                let mut missing_fields = vec![];
                for field in &self.required {
                    match o.get(field.as_str()) {
                        None => {
                            missing_fields.push(field.clone());
                        }
                        Some(&FieldValue::Null) => {
                            missing_fields.push(field.clone());
                        }
                        _ => {}
                    }
                }

                if !missing_fields.is_empty() {
                    Err(anyhow!(format!(
                        "[{}] field(s) are required ({})",
                        missing_fields.join(", "),
                        "Required"
                    )))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::constraint::object::required::Required;
    use crate::core::constraint::Constraint;
    use crate::core::value::{FieldValue, Reflect};
    use std::collections::BTreeMap;

    #[test]
    fn test_required() {
        struct Document {
            title: String,
        }

        impl Reflect for Document {
            fn field_value(&self) -> FieldValue {
                FieldValue::Object(BTreeMap::from([(
                    "title".to_string(),
                    self.title.field_value(),
                )]))
            }
        }

        let document = Document {
            title: "Document Title".to_string(),
        };

        let constraint = Required {
            required: vec!["title".to_string()],
        };
        assert!(constraint.validate(&document.field_value()).is_ok());

        let constraint = Required {
            required: vec!["title".to_string(), "body".to_string()],
        };
        assert!(constraint.validate(&document.field_value()).is_err());
    }
}
