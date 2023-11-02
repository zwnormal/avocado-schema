use avocado_schema::core::value::{FieldValue, Reflect};
use avocado_schema_derive::Reflect;
use std::collections::BTreeMap;

#[derive(Reflect)]
struct Client {
    #[reflect(name = "firstName")]
    first_name: String,
    #[reflect(name = "lastName")]
    last_name: String,
    age: u64,
}

#[test]
fn main() {
    let client = Client {
        first_name: "Robert".to_string(),
        last_name: "Li".to_string(),
        age: 30,
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
