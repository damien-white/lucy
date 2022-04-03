use nom::branch::alt;
use nom::character::complete::{alphanumeric1, char, one_of};
use nom::combinator::cut;
use nom::error::context;
use nom::sequence::{delimited, preceded, terminated};
use nom::{
    bytes::complete::{escaped, is_not, tag, take_while},
    combinator::value,
    IResult,
};

pub fn boolean(i: &[u8]) -> IResult<&[u8], bool> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(i)
}

pub fn nullish(i: &[u8]) -> IResult<&[u8], ()> {
    value((), tag("null"))(i)
}

// TODO: Start here where you left off and begin writing tests
pub fn string<'a>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    let string_inner = move |i: &'a [u8]| -> IResult<&[u8], &[u8]> {
        escaped(alphanumeric1, '\\', one_of("\"n\\"))(i)
    };

    context(
        "string",
        preceded(char('\"'), cut(terminated(string_inner, char('\"')))),
    )(i)
}

pub fn object(i: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(tag("{"), is_not("}"), tag("}"))(i)
}

// pub fn array(i: &[u8]) -> IResult<&[u8], &[u8]> {}

// pub fn number(i: &[u8]) -> IResult<&[u8], f32> {}

pub fn whitespace(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let tokens = [b' ', b'\t', b'\r', b'\n'].as_slice();

    take_while(move |t: u8| tokens.contains(&t))(i)
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

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
