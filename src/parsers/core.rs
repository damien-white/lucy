//! Parsers for the core JSON primitive and structural types.

use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, take_while, take_while1};
use nom::character::complete::{char as token, one_of};
use nom::combinator::{cut, map, value};
use nom::multi::separated_list0;
use nom::number::complete::double;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::types::Value;

/// Parses a JSON value with the `array` type.
pub fn array(input: &str) -> IResult<&str, Vec<Value>> {
    preceded(
        token('['),
        cut(terminated(
            separated_list0(preceded(whitespace, token(',')), json_value),
            preceded(whitespace, token(']')),
        )),
    )(input)
}

/// Parses a JSON value with the `boolean` type.
pub fn boolean(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

/// Parses a JSON value with the `object` type.
pub fn object(input: &str) -> IResult<&str, HashMap<&str, Value>> {
    preceded(
        token('{'),
        cut(terminated(
            map(
                separated_list0(preceded(whitespace, token(',')), key_value_pair),
                |entries| collect_entries(&entries),
            ),
            preceded(whitespace, token('}')),
        )),
    )(input)
}

/// Parses a JSON value with the `null` type.
pub fn null(input: &str) -> IResult<&str, ()> {
    value((), tag("null"))(input)
}

/// Parses a JSON value with the `number` type.
pub fn number(input: &str) -> IResult<&str, f64> {
    double(input)
}

/// Parses a JSON value with the `string` type.
pub fn string(input: &str) -> IResult<&str, &str> {
    preceded(
        token('\"'),
        cut(terminated(
            escaped(take_while1(is_string_token), '\\', one_of("\"bfnrt\\")),
            token('\"'),
        )),
    )(input)
}

// ===== Utility functions and miscellaneous parsers =====
pub(crate) fn key_value_pair(input: &str) -> IResult<&str, (&str, Value)> {
    separated_pair(
        preceded(whitespace, string),
        cut(preceded(whitespace, token(':'))),
        json_value,
    )(input)
}

pub fn json_value(input: &str) -> IResult<&str, Value> {
    preceded(
        whitespace,
        alt((
            map(array, Value::Array),
            map(boolean, Value::Boolean),
            map(object, Value::Object),
            map(null, |_| Value::Null),
            map(number, Value::Number),
            map(string, Value::String),
        )),
    )(input)
}

#[inline]
fn collect_entries<'a>(entries: &[(&'a str, Value<'a>)]) -> HashMap<&'a str, Value<'a>> {
    HashMap::from_iter(entries.iter().cloned())
}

fn whitespace(input: &str) -> IResult<&str, &str> {
    take_while(is_whitespace)(input)
}

#[inline]
fn is_string_token(c: char) -> bool {
    c != '"' && c != '\\'
}

#[inline]
fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n'
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    use super::*;

    #[test]
    fn array_values() {
        assert_eq!(
            array("[5, 42, 97.5]abcd"),
            Ok((
                "abcd",
                vec![
                    Value::Number(5_f64),
                    Value::Number(42_f64),
                    Value::Number(97.5_f64),
                ]
            ))
        );
        assert_eq!(
            array("[true, \"lucy\", null]abcd"),
            Ok((
                "abcd",
                vec![Value::Boolean(true), Value::String("lucy"), Value::Null,]
            ))
        );
    }

    #[test]
    fn boolean_values() {
        assert_eq!(boolean("true\"\nxxx"), Ok(("\"\nxxx", true)));
        assert_eq!(boolean("falsexyz"), Ok(("xyz", false)));
        assert_eq!(
            boolean("abcdef"),
            Err(Err::Error(Error::new("abcdef", ErrorKind::Tag)))
        );
        assert_eq!(
            boolean("xyzfalse"),
            Err(Err::Error(Error::new("xyzfalse", ErrorKind::Tag)))
        );
        assert_eq!(boolean(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
    }

    #[test]
    fn nullish_values() {
        assert_eq!(null("nullabcd"), Ok(("abcd", ())));
        assert_eq!(
            null("abcdef"),
            Err(Err::Error(Error::new("abcdef", ErrorKind::Tag)))
        );
        assert_eq!(
            null("abcdnull"),
            Err(Err::Error(Error::new("abcdnull", ErrorKind::Tag)))
        );
    }

    #[test]
    fn number_values() {
        // let input = &"8".as_bytes_mut();
        let input = r#"8"#;
        let expected = 8.;
        assert_eq!(number(input), Ok(("", expected)));

        assert_eq!(number("-1.234E-12"), Ok(("", -1.234e-12)))
    }

    #[test]
    fn string_values() {
        assert_eq!(string("\"0123456789abcdef\""), Ok(("", "0123456789abcdef")));
        assert_eq!(
            string("\"0x00 0x01 0x04 0xDE 0xAD\""),
            Ok(("", "0x00 0x01 0x04 0xDE 0xAD"))
        );
        assert_eq!(
            string("\"'0x00 0x01 0x04 0xDE 0xAD'\""),
            Ok(("", "'0x00 0x01 0x04 0xDE 0xAD'"))
        );
    }

    #[test]
    fn whitespace_characters() {
        assert_eq!(whitespace("    {"), Ok(("{", "    ")));
        assert_eq!(whitespace("\nabcdef"), Ok(("abcdef", "\n")),);
        assert_eq!(whitespace("\t  \r\nabcdef"), Ok(("abcdef", "\t  \r\n")),);

        let input = "\r\n\r\n  {\"field\": \"abcdef\"}\r\n\r\n";
        assert_eq!(
            whitespace(input),
            Ok(("{\"field\": \"abcdef\"}\r\n\r\n", "\r\n\r\n  "))
        );
    }
}
