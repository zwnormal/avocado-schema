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
    use crate::core::value::FieldValue;
    use std::collections::BTreeMap;

    #[test]
    fn test_required() {
        #[derive(Clone)]
        struct Document {
            title: String,
        }

        impl From<Document> for FieldValue {
            fn from(value: Document) -> Self {
                FieldValue::Object(BTreeMap::from([(
                    "title".to_string(),
                    FieldValue::String(value.title),
                )]))
            }
        }

        let document = Document {
            title: "Document Title".to_string(),
        };

        let constraint = Required {
            required: vec!["title".to_string()],
        };
        assert!(constraint.validate(&document.clone().into()).is_ok());

        let constraint = Required {
            required: vec!["title".to_string(), "body".to_string()],
        };
        assert!(constraint.validate(&document.into()).is_err());
    }
}
