# Avocado Schema Derive

Avocado Schema defines the following enum `FieldValue` for runtime reflection of `struct`'s structure and values:

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/badge/crates-0.6.3-blue
[crates-url]: https://crates.io/crates/avocado-schema-derive
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/zwnormal/avocado-schema/blob/main/LICENSE

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    String(String),
    Integer(i64),
    UInteger(u64),
    Float(f64),
    Boolean(bool),
    Object(BTreeMap<String, FieldValue>),
    Array(Vec<FieldValue>),
    Email(EmailAddress),
    DateTime(DateTime<Utc>),
    Date(NaiveDate),
    Time(NaiveTime),
    Null,
}
```

This macro `Reflect` is for deriving `FieldValue` enum for `struct`:

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
