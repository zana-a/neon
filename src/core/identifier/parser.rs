use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::sequence::tuple;

use crate::core::result::parser::Result;

#[derive(Debug, PartialEq)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(s: impl Into<String>) -> Self {
        Identifier(s.into())
    }
}

pub fn parse(s: &str) -> Result<&str, Identifier> {
    let letter_or_underscore = alt((tag("_"), alpha1));
    let letter_or_underscore_or_digit = many0(alt((alpha1, tag("_"), digit1)));

    match tuple((letter_or_underscore, letter_or_underscore_or_digit))(s) {
        Ok(result) => Ok((
            result.0,
            Identifier::new(format!("{}{}", result.1.0, result.1.1.join(""))),
        )),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::identifier::parser::*;

    #[test]
    fn test_parse_letter() {
        let (_, actual) = parse("a").unwrap();
        let expected = Identifier::new("a");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_underscore() {
        let (_, actual) = parse("_").unwrap();
        let expected = Identifier::new("_");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_letter_many() {
        let (_, actual) = parse("aaa").unwrap();
        let expected = Identifier::new("aaa");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_underscore_many() {
        let (_, actual) = parse("___").unwrap();
        let expected = Identifier::new("___");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_letter_underscore_many() {
        let (_, actual) = parse("a__").unwrap();
        let expected = Identifier::new("a__");
        assert_eq!(expected, actual);

        let (_, actual) = parse("_a_").unwrap();
        let expected = Identifier::new("_a_");
        assert_eq!(expected, actual);

        let (_, actual) = parse("__a").unwrap();
        let expected = Identifier::new("__a");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_letter_underscore_digit_many() {
        let (_, actual) = parse("a123").unwrap();
        let expected = Identifier::new("a123");
        assert_eq!(expected, actual);

        let (_, actual) = parse("a_1_2_3").unwrap();
        let expected = Identifier::new("a_1_2_3");
        assert_eq!(expected, actual);

        let (_, actual) = parse("a_b1_c2_d3").unwrap();
        let expected = Identifier::new("a_b1_c2_d3");
        assert_eq!(expected, actual);
    }
}
