# Avocado Schema

Inspired by the JSON schema, Avocado Schema define a schema DSL which can be interpreted to implement multiple purposes (like perform validation of the data, or generate GUIs dynamically).

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![codecov][codecov-badge]][codecov-url]

[crates-badge]: https://img.shields.io/badge/crates-0.5.0-blue
[crates-url]: https://crates.io/crates/avocado-schema
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/zwnormal/avocado-schema/blob/main/LICENSE
[codecov-badge]: https://codecov.io/gh/zwnormal/avocado-schema/graph/badge.svg?token=D3NUTKPBYM
[codecov-url]: https://codecov.io/gh/zwnormal/avocado-schema

## How to use

Please refer to the sources/tests code in for both how to write a visitor and how to validate data by the schema.

If any error occurs, the error will be returned in the format of `HashMap<String, Vec<ValidationError>>`. The key of the `HashMap` is path to the field where has validation error, and the `ValidationError` just contains the message of the error.

Besides creating the schema based on json, the `builder` pattern is also implemented to build the schema by code.

Due to the nature of rust `serde`, it is not hard to support other file format (like XML) for defining the schema.
