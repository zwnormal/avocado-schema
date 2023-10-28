# Avocado Schema

Inspired by the JSON schema, the main purpose of Avocado Schema is to avoid defining a static schema or validation in macro, so with a flexible, separate schema defined with, for example, json string, the schema can be changed/inspected and/or saved into/load from, for example, database. Meanwhile, it can also be interpreted to implement multiple purposes (like perform validation of the data, or generate GUIs dynamically).

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![codecov][codecov-badge]][codecov-url]

[crates-badge]: https://img.shields.io/badge/crates-0.5.1-blue
[crates-url]: https://crates.io/crates/avocado-schema
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/zwnormal/avocado-schema/blob/main/LICENSE
[codecov-badge]: https://codecov.io/gh/zwnormal/avocado-schema/graph/badge.svg?token=D3NUTKPBYM
[codecov-url]: https://codecov.io/gh/zwnormal/avocado-schema

## How to use

The src/core/value.rs defines an `FieldValue` enum to implement the reflection of struct value, so any struct that requires to be validated against the schema needs to implement the `from` trait for the `FieldValue`. Several useful implementation has been already included in the file.

Please refer to the sources/tests code for both how to write a visitor and how to validate data by the schema.

If any error occurs, the error will be returned in the format of `HashMap<String, Vec<ValidationError>>`. The key of the `HashMap` is path to the field where has validation error, and the `ValidationError` just contains the message of the error.

Besides creating the schema based on json, the `builder` pattern is also implemented to build the schema by code.

Due to the nature of rust `serde`, it is not hard to support other file format (like XML) for defining the schema.
