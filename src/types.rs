use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Type {
    /// Primitive `string` type
    String(String),
    /// Primitive `number` type
    Number(f32),
    /// Primitive `boolean` type
    Boolean(bool),
    /// Primitive `null` type
    Null,
    /// Structured `object` type
    Object(HashMap<String, Type>),
    /// Structured `array` type
    Array(Vec<Type>),
}
