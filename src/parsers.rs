use nom::branch::alt;
use nom::bytes::complete::{is_not, take_while};
// use nom::error::ParseError;
use nom::sequence::delimited;
use nom::{bytes::complete::tag, combinator::value, IResult};

pub fn boolean(input: &[u8]) -> IResult<&[u8], bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

pub fn nullish(input: &[u8]) -> IResult<&[u8], ()> {
    value((), tag("null"))(input)
}

pub fn object(input: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(tag("{"), is_not("}"), tag("}"))(input)
}

pub fn whitespace(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let tokens = " \t\r\n";
    take_while(move |token| tokens.contains(token as char))(input)
}

#[cfg(test)]
mod tests {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };

    use super::*;

    #[test]
    fn parse_boolean_works() {
        assert_eq!(boolean(&b"true\"\nxxx"[..]), Ok((&b"\"\nxxx"[..], true)));
        assert_eq!(
            boolean(&b"asdf"[..]),
            Err(nom::Err::Error(nom::error::Error::new(
                &b"asdf"[..],
                nom::error::ErrorKind::Tag,
            )))
        );
        assert_eq!(boolean(&b"falsexyz"[..]), Ok((&b"xyz"[..], false)));

        assert_eq!(
            boolean(&b"xyzfalse"[..]),
            Err(Err::Error(Error::new(&b"xyzfalse"[..], ErrorKind::Tag)))
        );
        assert_eq!(
            boolean(&b"null"[..]),
            Err(nom::Err::Error(nom::error::Error::new(
                &b"null"[..],
                nom::error::ErrorKind::Tag,
            )))
        );
        assert_eq!(
            boolean(&b""[..]),
            Err(nom::Err::Error(nom::error::Error::new(
                &b""[..],
                nom::error::ErrorKind::Tag,
            )))
        );
    }
}
