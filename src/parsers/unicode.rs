//! Parsers and functions specific to escaped unicode sequences.

use nom::bytes::complete::take_while_m_n;
use nom::character::complete::char as token;
use nom::combinator::map_res;
use nom::sequence::{delimited, preceded};
use nom::IResult;

// TODO: Flesh POC parser out so it can handle escaped unicode sequences
//       Escaped sequences will help produce actual unicode characters
pub fn unicode_sequence(input: &str) -> IResult<&str, u32> {
    let hex_digits = take_while_m_n(1, 6, |ch: char| ch.is_ascii_hexdigit());
    map_res(
        preceded(token('u'), delimited(token('{'), hex_digits, token('}'))),
        |val| u32::from_str_radix(val, 16),
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    use super::*;

    #[test]
    fn unicode_values_in_strings() {
        assert_eq!(unicode_sequence("u{005C}XYZ"), Ok(("XYZ", 92)));
        assert_eq!(unicode_sequence("u{7F80}"), Ok(("", 32640)));
        assert_eq!(
            unicode_sequence("a\\b"),
            Err(Err::Error(Error::new("a\\b", ErrorKind::Char)))
        );
    }
}
