//! Parsers for the JSON primitive and structural types.
use std::collections::HashMap;

use nom::bytes::complete::{take_while, take_while1};
use nom::combinator::map_res;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{char as char_tag, one_of},
    combinator::value,
    combinator::{cut, map},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::types::Value;

pub fn array(input: &[u8]) -> IResult<&[u8], Vec<Value>> {
    preceded(
        char_tag('['),
        cut(terminated(
            separated_list0(preceded(whitespace, char_tag(',')), json_value),
            preceded(whitespace, char_tag(']')),
        )),
    )(input)
}

pub fn boolean(input: &[u8]) -> IResult<&[u8], bool> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(input)
}

pub fn object(input: &[u8]) -> IResult<&[u8], HashMap<&str, Value>> {
    preceded(
        char_tag('{'),
        cut(terminated(
            map(
                separated_list0(preceded(whitespace, char_tag(',')), key_value_pair),
                |tuple_vec| tuple_vec.into_iter().map(|(k, v)| (k, v)).collect(),
            ),
            preceded(whitespace, char_tag('}')),
        )),
    )(input)
}

pub fn null(input: &[u8]) -> IResult<&[u8], ()> {
    value((), tag("null"))(input)
}

// pub fn number(input: &[u8]) -> IResult<&[u8], f32> {}

pub fn string(input: &[u8]) -> IResult<&[u8], &str> {
    preceded(
        char_tag('\"'),
        cut(terminated(
            map_res(
                escaped(take_while1(is_string_char), '\\', one_of("\"bfnrt\\")),
                std::str::from_utf8,
            ),
            char_tag('\"'),
        )),
    )(input)
}

// ===== Utility functions and miscellaneous parsers =====
pub fn key_value_pair(input: &[u8]) -> IResult<&[u8], (&str, Value)> {
    separated_pair(
        preceded(whitespace, string),
        cut(preceded(whitespace, char_tag(':'))),
        json_value,
    )(input)
}

pub fn json_value(input: &[u8]) -> IResult<&[u8], Value> {
    preceded(
        whitespace,
        alt((
            map(array, Value::Array),
            map(boolean, Value::Boolean),
            map(object, Value::Object),
            map(null, |_| Value::Null),
            map(string, Value::String),
        )),
    )(input)
}

pub fn whitespace(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_whitespace)(input)
}

#[inline]
fn is_string_char(c: u8) -> bool {
    c != b'"' && c != b'\\'
}

#[inline]
fn is_whitespace(ch: u8) -> bool {
    ch == b' ' || ch == b'\t' || ch == b'\r' || ch == b'\n'
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};

    use super::*;

    #[test]
    fn boolean_types() {
        assert_eq!(boolean(&b"true\"\nxxx"[..]), Ok((&b"\"\nxxx"[..], true)));
        assert_eq!(boolean(&b"falsexyz"[..]), Ok((&b"xyz"[..], false)));
        assert_eq!(
            boolean(&b"abcdef"[..]),
            Err(nom::Err::Error(Error::new(&b"abcdef"[..], ErrorKind::Tag)))
        );
        assert_eq!(
            boolean(&b"xyzfalse"[..]),
            Err(nom::Err::Error(Error::new(
                &b"xyzfalse"[..],
                ErrorKind::Tag,
            )))
        );
        assert_eq!(
            boolean(&b""[..]),
            Err(nom::Err::Error(Error::new(&b""[..], ErrorKind::Tag)))
        );
    }

    #[test]
    fn nullish_types() {
        assert_eq!(null(&b"nullabcd"[..]), Ok((&b"abcd"[..], ())));
        assert_eq!(
            null(&b"abcdef"[..]),
            Err(nom::Err::Error(Error::new(&b"abcdef"[..], ErrorKind::Tag)))
        );
        assert_eq!(
            null(&b"abcdnull"[..]),
            Err(nom::Err::Error(Error::new(
                &b"abcdnull"[..],
                ErrorKind::Tag,
            )))
        );
    }

    #[test]
    fn string_types() {
        assert_eq!(
            string(r#""0123456789abcdef""#.as_bytes()),
            Ok((&b""[..], "0123456789abcdef"))
        );
        assert_eq!(
            string(&br#""0x00 0x01 0x04 0xDE 0xAD""#[..]),
            Ok((&b""[..], "0x00 0x01 0x04 0xDE 0xAD"))
        );
        assert_eq!(
            string(&br#""'0x00 0x01 0x04 0xDE 0xAD'""#[..]),
            Ok((&b""[..], "'0x00 0x01 0x04 0xDE 0xAD'"))
        );
    }

    #[test]
    fn whitespace_characters() {
        assert_eq!(whitespace(&b"    {"[..]), Ok((&b"{"[..], &b"    "[..])));
        assert_eq!(
            whitespace(&b"\nabcdef"[..]),
            Ok((&b"abcdef"[..], &b"\n"[..])),
        );
        assert_eq!(
            whitespace(&b"\t  \r\nabcdef"[..]),
            Ok((&b"abcdef"[..], &b"\t  \r\n"[..])),
        );

        let input = &b"\r\n\r\n  {\"field\": \"abcdef\"}\r\n\r\n"[..];
        assert_eq!(
            whitespace(input),
            Ok((&b"{\"field\": \"abcdef\"}\r\n\r\n"[..], &b"\r\n\r\n  "[..]))
        );
    }
}
