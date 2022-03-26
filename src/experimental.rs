#![allow(dead_code)]

use nom::error::ParseError;
use nom::{IResult, Parser};

fn parse_hello_prefix(input: &str) -> IResult<&str, &str, ()> {
    match input.strip_prefix("Hello") {
        Some(tail) => Ok((tail, "Hello")),
        None => Err(nom::Err::Error(())),
    }
}

/// Generic string tag parser combinator.
fn parse_tag<'i: 't, 't>(tag: &'t str) -> impl Parser<&'i str, &'i str, ()> + 't {
    move |input: &'i str| match input.strip_prefix(tag) {
        Some(tail) => Ok((tail, &input[..tag.len()])),
        None => Err(nom::Err::Error(())),
    }
}

fn parse_comma_tags<'i: 't, 't>(
    tag1: &'t str,
    tag2: &'t str,
) -> impl Parser<&'i str, (&'i str, &'i str), ()> + 't {
    let mut parse_tag1 = parse_tag(tag1);
    let mut parse_delimiter = parse_tag(", ");
    let mut parse_tag2 = parse_tag(tag2);
    move |input: &'i str| {
        let (tail, value1) = parse_tag1.parse(input)?;
        let (tail, _) = parse_delimiter.parse(tail)?;
        let (tail, value2) = parse_tag2.parse(tail)?;
        Ok((tail, (value1, value2)))
    }
}

fn parse_separated_tags<'i: 't, 't>(
    tag1: &'t str,
    tag2: &'t str,
    delimiter: &'t str,
) -> impl Parser<&'i str, (&'i str, &'i str), ()> + 't {
    move |input: &'i str| {
        let (tail, value1) = parse_tag(tag1).parse(input)?;
        let (tail, _) = parse_tag(delimiter).parse(tail)?;
        let (tail, value2) = parse_tag(tag2).parse(tail)?;
        Ok((tail, (value1, value2)))
    }
}

fn parse_separated<I, O1, O2, S, E>(
    mut parse_tag1: impl Parser<I, O1, E>,
    mut parse_delimiter: impl Parser<I, S, E>,
    mut parse_tag2: impl Parser<I, O2, E>,
) -> impl Parser<I, (O1, O2), E> {
    move |input: I| {
        let (tail, value1) = parse_tag1.parse(input)?;
        let (tail, _) = parse_delimiter.parse(tail)?;
        let (tail, value2) = parse_tag2.parse(tail)?;
        Ok((tail, (value1, value2)))
    }
}

fn parse_bool(input: &str) -> IResult<&str, bool, ()> {
    match parse_tag("true").parse(input) {
        Ok((tail, _)) => Ok((tail, true)),
        // This here throws away useful error information. The branch that
        // discards the error information contains the `_err` match arm var
        Err(nom::Err::Error(_err)) => match parse_tag("false").parse(input) {
            Ok((tail, _)) => Ok((tail, false)),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn parse_either<I, O, E>(
    mut first: impl Parser<I, O, E>,
    mut second: impl Parser<I, O, E>,
) -> impl Parser<I, O, E>
where
    I: Clone,
    E: ParseError<I>,
{
    move |input: I| match first.parse(input.clone()) {
        Ok((tail, value)) => Ok((tail, value)),
        Err(nom::Err::Error(err1)) => match second.parse(input) {
            Ok((tail, value)) => Ok((tail, value)),
            Err(nom::Err::Error(err2)) => Err(nom::Err::Error(err1.or(err2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // N.B. This parser works in a vacuum but contains a significant bug. It throws away any errors!
    #[test]
    fn parse_boolean_works() {
        assert_eq!(parse_bool("true, 0123456789"), Ok((", 0123456789", true)));
        assert_eq!(parse_bool("false, 0123456789"), Ok((", 0123456789", false)));
        assert!(parse_bool("0123456789, hello").is_err());
    }

    // TODO: Add proper error handling as failures are currently not recoverable
    #[test]
    fn parse_either_works_properly() {
        // let err_msg = "error handling not yet implemented";
        let mut parser = parse_either(parse_tag("true"), parse_tag("false"));
        assert_eq!(
            parser.parse("true, 0123456789"),
            Ok((", 0123456789", "true"))
        );
        assert_eq!(
            parser.parse("false, 0123456789"),
            Ok((", 0123456789", "false"))
        );
        assert!(parser.parse("Hello, World!").is_err());
        assert!(parser.parse("0123456789, true").is_err());

        assert!(parser.parse("1, 0123456789").is_err());
    }

    #[test]
    fn top_level_parser_works() {
        let mut parser = parse_separated(parse_tag("Hello"), parse_tag(", "), parse_tag("World"));
        assert_eq!(parser.parse("Hello, World!"), Ok(("!", ("Hello", "World"))));
    }

    #[test]
    fn parse_hello_prefix_works() {
        assert_eq!(
            parse_tag("Hello").parse("Hello, World!"),
            Ok((", World!", "Hello"))
        );
    }

    #[test]
    fn parse_comma_tags_succeeds() {
        assert_eq!(
            parse_comma_tags("Hello", "World").parse("Hello, World!"),
            Ok(("!", ("Hello", "World")))
        );
        assert_eq!(
            parse_comma_tags("I love you", "Josephine").parse("I love you, Josephine"),
            Ok(("", ("I love you", "Josephine")))
        );
        assert_eq!(
            parse_comma_tags("6", "value1").parse("6, value14, key1"),
            Ok(("4, key1", ("6", "value1")))
        );
    }

    #[test]
    fn parse_separated_tags_succeeds() {
        assert_eq!(
            parse_separated_tags("7", "3bhCoiL", ":").parse("7:3bhCoiL7:BCDHhgk"),
            Ok(("7:BCDHhgk", ("7", "3bhCoiL")))
        )
    }
}
