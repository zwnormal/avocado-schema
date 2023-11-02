use avocado_schema::core::value::{FieldValue, Reflect};
use avocado_schema_derive::Reflect;
use std::collections::BTreeMap;

#[derive(Reflect)]
struct Client {
    first_name: String,
    last_name: String,
}

#[test]
fn main() {
    let client = Client {
        first_name: "Robert".to_string(),
        last_name: "Li".to_string(),
    };
    assert_eq!(
        client.field_value(),
        FieldValue::Object(BTreeMap::from([
            (
                "first_name".to_string(),
                FieldValue::String("Robert".to_string())
            ),
            (
                "last_name".to_string(),
                FieldValue::String("Li".to_string())
            )
        ]))
    )
}
