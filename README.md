# Avocado Schema

Inspired by the JSON schema, Avocado Schema define a schema DSL which can be interpreted to implement multiple purposes (like perform validation of the data, or generate GUIs dynamically).

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![codecov][codecov-badge]][codecov-url]

[crates-badge]: https://img.shields.io/badge/crates-0.2.8-blue
[crates-url]: https://crates.io/crates/avocado-schema
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/zwnormal/avocado-schema/blob/main/LICENSE
[codecov-badge]: https://codecov.io/gh/zwnormal/avocado-schema/graph/badge.svg?token=D3NUTKPBYM
[codecov-url]: https://codecov.io/gh/zwnormal/avocado-schema

## How to use

Please refer to the test code in src/visitor/validator.rs for both how to write a visitor and how to validate data by the schema.

The example defines the schema directly in `json` format, and then deserialize directly into `struct`, and then create a validator based on the schema:
```rust
use avocado_schema::core::field::object::ObjectField;
use avocado_schema::visitor::validator::Validator;
use serde::Serialize;

#[test]
fn test_validate() {
    #[derive(Serialize)]
    struct Client {
        first_name: String,
        last_name: String,
        age: u64,
    }

    let schema_json = r#"
    {
        "type":"object",
        "name": "client",
        "title": "Client",
        "properties": {
            "first_name": {
                "type": "string",
                "name": "first_name",
                "title": "First Name",
                "max_length": 32,
                "min_length": 8
            },
            "last_name": {
                "type": "string",
                "name": "last_name",
                "title": "Last Name",
                "max_length": 32,
                "min_length": 8
            },
            "age": {
                "type": "integer",
                "name": "age",
                "title": "Age",
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
```

If any error occurs, the error will be returned in the format of `HashMap<String, Vec<ValidationError>>`. The key of the `HashMap` is path to the field where has validation error, and the `ValidationError` just contains the message of the error.

Besides creating the schema based on json, the `builder` pattern is also implemented to build the schema by code.

Due to the nature of rust `serde`, it is not hard to support other file format (like XML) for defining the schema.
