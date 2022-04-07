//! This module holds JSON type definitions and related utility functions.
use std::collections::HashMap;
use std::fmt::Debug;

/// The `Map<'a, V>` type is an alias for the JSON `object` type.
pub(crate) type Map<'a, V = Type<'a>> = HashMap<&'a str, V>;

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    /// Primitive `string` type
    String(&'a str),
    /// Primitive `number` type
    Number(f32),
    /// Primitive `boolean` type
    Boolean(bool),
    /// Primitive `null` type
    Null,
    /// Structured `object` type
    Object(Map<'a>),
    /// Structured `array` type
    Array(Vec<Type<'a>>),
}
