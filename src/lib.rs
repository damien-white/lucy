use nom::{bytes::complete::tag, combinator::value, IResult};

pub fn parse_true(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, value) = value(true, tag("true"))(input)?;
    Ok((input, value))
}

pub fn parse_false(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, value) = value(true, tag("false"))(input)?;
    Ok((input, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_true_works() {
        let input1 = "true, world".as_bytes();
        let input2 = "asdf, world".as_bytes();
        let input3 = "false, world".as_bytes();
        let input4 = "null, world".as_bytes();
        let input5 = "true".as_bytes();

        assert_eq!(parse_true(input1), Ok((&b", world"[..], true)));
        assert_eq!(
            parse_true(input2),
            Err(nom::Err::Error(nom::error::Error::new(
                &b"asdf, world"[..],
                nom::error::ErrorKind::Tag,
            )))
        );
        assert_eq!(
            parse_true(input3),
            Err(nom::Err::Error(nom::error::Error::new(
                &b"false, world"[..],
                nom::error::ErrorKind::Tag,
            )))
        );
        assert_eq!(
            parse_true(input4),
            Err(nom::Err::Error(nom::error::Error::new(
                &b"null, world"[..],
                nom::error::ErrorKind::Tag,
            )))
        );
        assert_eq!(parse_true(input5), Ok((&b""[..], true)));

        // assert_eq!(parse_boolean(iter.next(), )
    }
}
