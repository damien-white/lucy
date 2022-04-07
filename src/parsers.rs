use std::collections::HashMap;

use nom::bytes::complete::take_while;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{alphanumeric1, char as char_tag, one_of},
    combinator::value,
    combinator::{cut, map},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::types::Type;

pub fn boolean(i: &[u8]) -> IResult<&[u8], bool> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(i)
}

pub fn nullish(i: &[u8]) -> IResult<&[u8], ()> {
    value((), tag("null"))(i)
}

pub fn string<'a>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    let string_inner = move |inner: &'a [u8]| -> IResult<&[u8], &[u8]> {
        escaped(alphanumeric1, '\\', one_of("\"n\\"))(inner)
    };

    preceded(
        char_tag('\"'),
        cut(terminated(string_inner, char_tag('\"'))),
    )(i)
}

pub fn key_value_pair(i: &[u8]) -> IResult<&[u8], (&[u8], Type)> {
    separated_pair(
        preceded(whitespace, string),
        cut(preceded(whitespace, char_tag(':'))),
        json_value,
    )(i)
}

pub fn object(i: &[u8]) -> IResult<&[u8], HashMap<&str, Type>> {
    preceded(
        char_tag('{'),
        cut(terminated(
            map(
                separated_list0(preceded(whitespace, char_tag(',')), key_value_pair),
                |tuple_vec| {
                    tuple_vec
                        .into_iter()
                        .map(|(k, v)| (std::str::from_utf8(k).unwrap_or_default(), v))
                        .collect()
                },
            ),
            preceded(whitespace, char_tag('}')),
        )),
    )(i)
}

pub fn array(i: &[u8]) -> IResult<&[u8], Vec<Type>> {
    preceded(
        char_tag('['),
        cut(terminated(
            separated_list0(preceded(whitespace, char_tag(',')), json_value),
            preceded(whitespace, char_tag(']')),
        )),
    )(i)
}

// pub fn number(i: &[u8]) -> IResult<&[u8], f32> {}

pub fn json_value(i: &[u8]) -> IResult<&[u8], Type> {
    preceded(
        whitespace,
        alt((
            map(array, Type::Array),
            map(boolean, Type::Boolean),
            map(nullish, |_| Type::Null),
            map(string, |val| {
                Type::String(std::str::from_utf8(val).unwrap_or_default())
            }),
        )),
    )(i)
}

pub fn whitespace(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_whitespace)(i)
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
    fn boolean_values() {
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
    fn nullish_values() {
        assert_eq!(nullish(&b"nullabcd"[..]), Ok((&b"abcd"[..], ())));
        assert_eq!(
            nullish(&b"abcdef"[..]),
            Err(nom::Err::Error(Error::new(&b"abcdef"[..], ErrorKind::Tag)))
        );
        assert_eq!(
            nullish(&b"abcdnull"[..]),
            Err(nom::Err::Error(Error::new(
                &b"abcdnull"[..],
                ErrorKind::Tag,
            )))
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
