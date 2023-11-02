use avocado_schema_derive::Reflect;

#[derive(Reflect)]
struct Client {
    first_name: String,
    second_name: String,
}

#[test]
fn main() {}
