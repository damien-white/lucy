use nom::{IResult, Parser};

pub fn parse_hello_prefix(input: &str) -> IResult<&str, &str, ()> {
    match input.strip_prefix("Hello") {
        Some(tail) => Ok((tail, "Hello")),
        None => Err(nom::Err::Error(())),
    }
}

/// Generic string tag parser combinator.
pub fn parse_tag<'i: 't, 't>(tag: &'t str) -> impl Parser<&'i str, &'i str, ()> + 't {
    move |input: &'i str| match input.strip_prefix(tag) {
        Some(tail) => Ok((tail, &input[..tag.len()])),
        None => Err(nom::Err::Error(())),
    }
}

pub fn parse_comma_tags<'i: 't, 't>(
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

pub fn parse_separated_tags<'i: 't, 't>(
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

pub fn parse_separated<I, O1, O2, S, E>(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_level_parser_works() {
        let mut parse_hello_world =
            parse_separated(parse_tag("Hello"), parse_tag(", "), parse_tag("World"));
        assert_eq!(
            parse_hello_world.parse("Hello, World!"),
            Ok(("!", ("Hello", "World")))
        );
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
