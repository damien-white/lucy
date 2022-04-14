//! JSON type definitions used to construct Rust types.
use std::collections::HashMap;
use std::fmt::Debug;

/// This enum represents all valid [JSON data types][types].
///
/// various data types defined in the official
/// specification for the JSON format.
///
/// [types]: https://datatracker.ietf.org/doc/html/rfc8259
#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    /// Structured `array` type
    Array(Vec<Value<'a>>),
    /// Primitive `boolean` type
    Boolean(bool),
    /// Structured `object` type
    Object(HashMap<&'a str, Value<'a>>),
    /// Primitive `null` type
    Null,
    /// Primitive `number` type
    Number(f64),
    /// Primitive `string` type
    String(&'a str),
}

impl<'a> Value<'a> {
    pub fn repr(&self) -> &'a str {
        match *self {
            Value::Array(_) => "array",
            Value::Boolean(_) => "boolean",
            Value::Object(_) => "object",
            Value::Null => "null",
            Value::Number(_) => "number",
            Value::String(_) => "string",
        }
    }
}
