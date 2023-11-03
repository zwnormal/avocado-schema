use crate::core::field::array::ArrayField;
use crate::core::field::object::ObjectField;
use crate::core::field::Field;
use crate::core::field::FieldEnum;
use crate::core::value::{FieldValue, Reflect};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ValidationError {}

struct State {
    value: FieldValue,
    field_names: Vec<String>,
    errors: HashMap<String, Vec<ValidationError>>,
}

#[derive(Debug)]
pub struct Validator {
    schema: FieldEnum,
}

impl Validator {
    fn report_error(&self, error: ValidationError, state: &mut State) {
        let field = state.field_names.clone().join("/");
        if state.errors.contains_key(field.as_str()) {
            state.errors.get_mut(field.as_str()).unwrap().push(error);
        } else {
            state.errors.insert(field, vec![error]);
        }
    }

    fn validate_field(&self, field: &(impl Field + ?Sized), state: &mut State) {
        state.field_names.push(field.name().clone());
        for constraint in field.constrains() {
            match constraint.validate(&state.value) {
                Ok(_) => {}
                Err(e) => {
                    self.report_error(
                        ValidationError {
                            message: e.to_string(),
                        },
                        state,
                    );
                }
            }
        }
        state.field_names.pop();
    }

    fn visit_array(&self, array: &ArrayField, state: &mut State) {
        self.validate_field(array, state);
        state.field_names.push(array.name().clone());
        if let FieldValue::Array(values) = state.value.clone() {
            if let Some(item) = &array.item {
                for value in values {
                    state.value = value;
                    self.visit(item, state);
                }
            }
        }
        state.field_names.pop();
    }

    fn visit_object(&self, object: &ObjectField, state: &mut State) {
        self.validate_field(object, state);
        state.field_names.push(object.name().clone());
        if let FieldValue::Object(o) = state.value.clone() {
            for (name, value) in o {
                if let Some(field) = object.properties.get(name.as_str()) {
                    state.value = value;
                    self.visit(field, state);
                };
            }
        }
        state.field_names.pop();
    }

    fn visit(&self, field: &FieldEnum, state: &mut State) {
        match field {
            FieldEnum::Array(f) => self.visit_array(f, state),
            FieldEnum::Boolean(f) => self.validate_field(f, state),
            FieldEnum::Float(f) => self.validate_field(f, state),
            FieldEnum::Integer(f) => self.validate_field(f, state),
            FieldEnum::UInteger(f) => self.validate_field(f, state),
            FieldEnum::Object(f) => self.visit_object(f, state),
            FieldEnum::String(f) => self.validate_field(f, state),
            FieldEnum::Email(f) => self.validate_field(f, state),
            FieldEnum::Datetime(f) => self.validate_field(f, state),
            FieldEnum::Date(f) => self.validate_field(f, state),
            FieldEnum::Time(f) => self.validate_field(f, state),
        }
    }

    pub fn new(field: impl Field) -> Self {
        Validator {
            schema: field.into(),
        }
    }

    pub fn validate(
        &self,
        value: &impl Reflect,
    ) -> Result<(), HashMap<String, Vec<ValidationError>>> {
        let mut state = State {
            value: value.field_value(),
            field_names: vec![],
            errors: Default::default(),
        };

        self.visit(&self.schema, &mut state);
        if state.errors.is_empty() {
            Ok(())
        } else {
            Err(state.errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::field::object::ObjectField;
    use crate::core::value::{FieldValue, Reflect};
    use crate::visitor::validator::Validator;
    use std::collections::BTreeMap;

    #[test]
    fn test_validate() {
        struct Client {
            first_name: String,
            last_name: String,
            age: u64,
        }

        impl Reflect for Client {
            fn field_value(&self) -> FieldValue {
                FieldValue::Object(BTreeMap::from([
                    ("first_name".to_string(), self.first_name.field_value()),
                    ("last_name".to_string(), self.last_name.field_value()),
                    ("age".to_string(), self.age.field_value()),
                ]))
            }
        }

        let schema_json = r#"
        {
            "type":"object",
            "name": "client",
            "properties": {
                "first_name": {
                    "type": "string",
                    "name": "first_name",
                    "max_length": 32,
                    "min_length": 8
                },
                "last_name": {
                    "type": "string",
                    "name": "last_name",
                    "max_length": 32,
                    "min_length": 8
                },
                "age": {
                    "type": "uinteger",
                    "name": "age",
                    "maximum": 200,
                    "minimum": 0
                }
            }
        }"#;
        let schema: ObjectField = serde_json::from_str(schema_json).unwrap();
        let validator = Validator::new(schema);

        let valid_client = Client {
            first_name: "Robert".to_string(),
            last_name: "Li".to_string(),
            age: 32,
        };
        assert!(validator.validate(&valid_client).is_ok());

        let invalid_client = Client {
            first_name: "Robert".to_string(),
            last_name: "Li".to_string(),
            age: 201,
        };
        let result = validator.validate(&invalid_client);
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .get("client/age")
            .unwrap()
            .get(0)
            .unwrap()
            .message
            .contains("value 201 is larger then 200 (Maximum)"));
    }
}
