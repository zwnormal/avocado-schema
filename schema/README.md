# Avocado Schema

Inspired by the JSON schema, the main purpose of Avocado Schema is to avoid defining a static schema or validation in macro, so with a flexible, separate schema defined with, for example, json string, the schema can be changed/inspected and/or saved into/load from, for example, database. Meanwhile, it can also be interpreted to implement multiple purposes (like perform validation of the data, or generate GUIs dynamically).

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![codecov][codecov-badge]][codecov-url]

[crates-badge]: https://img.shields.io/badge/crates-0.8.0-blue
[crates-url]: https://crates.io/crates/avocado-schema
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/zwnormal/avocado-schema/blob/main/LICENSE
[codecov-badge]: https://codecov.io/gh/zwnormal/avocado-schema/graph/badge.svg?token=D3NUTKPBYM
[codecov-url]: https://codecov.io/gh/zwnormal/avocado-schema

## How to use

The src/core/value.rs defines an `FieldValue` enum to implement the reflection of struct value, so any struct that requires to be validated against the schema needs to implement the `Reflect` trait. Several useful implementation has been already included in the file. The schema derive [crate](https://crates.io/crates/avocado-schema-derive) provides a derive macro for deriving the `FieldValue` for `struct`.

Please refer to the sources/tests code for both how to write a visitor and how to validate data by the schema. Here is a quick example:
```rust
#[derive(Reflect)]
struct Client {
    first_name: String,
    last_name: String,
    age: u64,
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
```

If any error occurs, the error will be returned in the format of `BTreeMap<String, Vec<ValidationError>>`. The key is path to the field where has validation error, and the `ValidationError` just contains the message of the error.

Besides creating the schema based on json, the `builder` pattern is also implemented to build the schema by code.

