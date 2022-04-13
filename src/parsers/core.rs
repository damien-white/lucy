//! Parsers for the core JSON primitive and structural types.

use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, take_while, take_while1};
use nom::character::complete::{char, one_of};
use nom::combinator::{cut, map, map_res, value};
use nom::multi::separated_list0;
use nom::number::complete::double;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::types::Value;

/// Parses a JSON value with the `array` type.
pub fn array(input: &[u8]) -> IResult<&[u8], Vec<Value>> {
    preceded(
        char('['),
        cut(terminated(
            separated_list0(preceded(whitespace, char(',')), json_value),
            preceded(whitespace, char(']')),
        )),
    )(input)
}

/// Parses a JSON value with the `boolean` type.
pub fn boolean(input: &[u8]) -> IResult<&[u8], bool> {
    alt((value(true, tag(b"true")), value(false, tag(b"false"))))(input)
}

/// Parses a JSON value with the `object` type.
pub fn object(input: &[u8]) -> IResult<&[u8], HashMap<&str, Value>> {
    preceded(
        char('{'),
        cut(terminated(
            map(
                separated_list0(preceded(whitespace, char(',')), key_value_pair),
                |entries| collect_entries(&entries),
            ),
            preceded(whitespace, char('}')),
        )),
    )(input)
}

/// Parses a JSON value with the `null` type.
pub fn null(input: &[u8]) -> IResult<&[u8], ()> {
    value((), tag("null"))(input)
}

/// Parses a JSON value with the `number` type.
pub fn number(input: &[u8]) -> IResult<&[u8], f64> {
    double(input)
    // let num = |num: &str| num.parse::<f64>();
    // map_res()(input)
}

/// Parses a JSON value with the `string` type.
pub fn string(input: &[u8]) -> IResult<&[u8], &str> {
    preceded(
        char('\"'),
        cut(terminated(
            map_res(
                escaped(take_while1(is_string_char), '\\', one_of("\"bfnrt\\")),
                std::str::from_utf8,
            ),
            char('\"'),
        )),
    )(input)
}

// ===== Utility functions and miscellaneous parsers =====
pub(crate) fn key_value_pair(input: &[u8]) -> IResult<&[u8], (&str, Value)> {
    separated_pair(
        preceded(whitespace, string),
        cut(preceded(whitespace, char(':'))),
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
            map(number, Value::Number),
            map(string, Value::String),
        )),
    )(input)
}

#[inline]
fn collect_entries<'a>(entries: &[(&'a str, Value<'a>)]) -> HashMap<&'a str, Value<'a>> {
    HashMap::from_iter(entries.iter().cloned())
}

fn whitespace(input: &[u8]) -> IResult<&[u8], &[u8]> {
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
    use nom::Err;

    use super::*;

    #[test]
    fn boolean_values() {
        assert_eq!(boolean(&b"true\"\nxxx"[..]), Ok((&b"\"\nxxx"[..], true)));
        assert_eq!(boolean(&b"falsexyz"[..]), Ok((&b"xyz"[..], false)));
        assert_eq!(
            boolean(&b"abcdef"[..]),
            Err(Err::Error(Error::new(&b"abcdef"[..], ErrorKind::Tag)))
        );
        assert_eq!(
            boolean(&b"xyzfalse"[..]),
            Err(Err::Error(Error::new(&b"xyzfalse"[..], ErrorKind::Tag)))
        );
        assert_eq!(
            boolean(&b""[..]),
            Err(Err::Error(Error::new(&b""[..], ErrorKind::Tag)))
        );
    }

    #[test]
    fn nullish_values() {
        assert_eq!(null(&b"nullabcd"[..]), Ok((&b"abcd"[..], ())));
        assert_eq!(
            null(&b"abcdef"[..]),
            Err(Err::Error(Error::new(&b"abcdef"[..], ErrorKind::Tag)))
        );
        assert_eq!(
            null(&b"abcdnull"[..]),
            Err(Err::Error(Error::new(&b"abcdnull"[..], ErrorKind::Tag)))
        );
    }

    #[test]
    fn number_values() {
        // let input = &"8".as_bytes_mut();
        let input = &br#"8"#[..];
        let expected = 8.;
        assert_eq!(number(input), Ok((&b""[..], expected)));

        assert_eq!(number(&b"-1.234E-12"[..]), Ok((&b""[..], -1.234e-12)))
    }

    #[test]
    fn string_values() {
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

    // #[test]
    // fn unicode_values_in_strings() {
    //     assert_eq!(string(&br#""a\\u005Cb"XYZ"#[..]), Ok((&b"XYZ"[..], "a\\b")));
    //     let data = "\"tab:\\tafter tab, newline:\\nnew line, quote: \\\", emoji: \\u{1F602}, newline:\\nescaped whitespace: \\    abc\"";
    //     println!(
    //         "EXAMPLE 2:\nParsing a string with escape sequences, newline literal, and escaped whitespace:\n\n{}\n",
    //         data
    //     );
    // }

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
