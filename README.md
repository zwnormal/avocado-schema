# Avocado Schema

Inspired by the JSON schema, Avocado Schema define a schema DSL which can be interpreted to implement multiple purpose (like perform validation of the data, or generate GUIs dynamically).

## How to use

The usage of Avocado Schema is very simple, please refer to the test code in src/visitor/validator.rs.

The example defines the schema directly in `json` format, and then deserialize directly into `struct`, and then create a validator based on the schema:
```rust
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
let mut validator = Validator::new(schema);
```
After the validator is created, it can be used to validate the data directly:
```rust
let valid_client = Client {
    first_name: "Robert".to_string(),
    last_name: "Li".to_string(),
    age: 32,
};
assert!(validator.validate(&valid_client).is_ok());
```

If any error occurs, the error will be returned in the format of `HashMap<String, Vec<ValidationError>>`. The key of the `HashMap` is path to the field where has validation error, and the `ValidationError` just contains the message of the error.

Besides creating the schema based on json, the `builder` pattern is also implemented to build the schema by code.

Due to the nature of rust `serde`, it is not hard to support other file format (like XML) for defining the schema.

## Things to Do

We can of course, add more constraints as we need, like to support more and more constraints defined in the JSON schema.

- [x] Add `format` of `Email` constraint for the `StringField`.
- [ ] Introduce a way to define custom constraints.
