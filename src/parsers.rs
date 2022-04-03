use nom::branch::alt;
use nom::bytes::complete::escaped;
use nom::bytes::complete::{is_not, take_while};
use nom::character::complete::{alphanumeric1, one_of};
use nom::sequence::delimited;
use nom::{bytes::complete::tag, combinator::value, IResult};

pub fn boolean(i: &[u8]) -> IResult<&[u8], bool> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(i)
}

pub fn nullish(i: &[u8]) -> IResult<&[u8], ()> {
    value((), tag("null"))(i)
}

pub fn string(i: &[u8]) -> IResult<&[u8], &[u8]> {
    escaped(alphanumeric1, '\\', one_of("\"n\\"))(i)
}

pub fn object(i: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(tag("{"), is_not("}"), tag("}"))(i)
}

pub fn whitespace(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let tokens = [b' ', b'\t', b'\r', b'\n'].as_slice();

    take_while(move |t: u8| tokens.contains(&t))(i)
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
