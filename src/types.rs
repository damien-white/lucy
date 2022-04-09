//! This module holds JSON type definitions and related utility functions.
use std::collections::HashMap;
use std::fmt::Debug;

/// The `Map<'a, V>` type is an alias for the JSON `object` type.
pub(crate) type Map<'a, V = Type<'a>> = HashMap<&'a str, V>;

/// This enum represents all valid [JSON data types][types].
///
/// various data types defined in the official
/// specification for the JSON format.
///
/// [types]: https://datatracker.ietf.org/doc/html/rfc8259
#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    /// Structured `array` type
    Array(Vec<Type<'a>>),
    /// Primitive `boolean` type
    Boolean(bool),
    /// Structured `object` type
    Object(Map<'a>),
    /// Primitive `null` type
    Null,
    /// Primitive `number` type
    Number(f32),
    /// Primitive `string` type
    String(&'a str),
}
