# Avocado Schema Derive

This repo provide the derive macro `Reflect` for Avocado Schema, so the `FieldValue` enum can be generated base on the definition of struct:

```rust
#[derive(Reflect)]
struct Client {
    #[reflect("firstName")]
    first_name: String,
    #[reflect("lastName")]
    last_name: String,
    age: u64,
    #[reflect(ignore)]
    email: String
}

#[test]
fn main() {
    let client = Client {
        first_name: "Robert".to_string(),
        last_name: "Li".to_string(),
        age: 30,
        email: "admin@avocado.com".to_string(),
    };
    assert_eq!(
        client.field_value(),
        FieldValue::Object(BTreeMap::from([
            (
                "firstName".to_string(),
                FieldValue::String("Robert".to_string())
            ),
            ("lastName".to_string(), FieldValue::String("Li".to_string())),
            ("age".to_string(), FieldValue::UInteger(30))
        ]))
    )
}
```
