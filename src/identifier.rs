use nom::branch::alt;
use nom::combinator::all_consuming;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::pair;

use crate::prelude::parser;
use crate::prelude::parser::alpha;
use crate::prelude::parser::numeric;
use crate::prelude::parser::underscore;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn new(value: impl Into<String>) -> Self {
        Identifier {
            value: value.into(),
        }
    }
}

pub fn identifier(s: &str) -> parser::Result<&str, Identifier> {
    let alpha_underscore = alt((alpha, underscore));
    let alpha_numeric_underscore = opt(many1(alt((alpha, numeric, underscore))));

    recognize(all_consuming(pair(
        alpha_underscore,
        alpha_numeric_underscore,
    )))(s)
    .map(|(remaining, value)| (remaining, Identifier::new(value)))
}

#[cfg(test)]
mod tests {
    use crate::identifier::identifier;
    use crate::identifier::Identifier;

    #[test]
    fn test_underscore() {
        let expected = Ok(("", Identifier::new("_")));
        let actual = identifier("_");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_single_alpha() {
        let expected = Ok(("", Identifier::new("a")));
        let actual = identifier("a");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multi_alpha() {
        let expected = Ok(("", Identifier::new("abcdefg")));
        let actual = identifier("abcdefg");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_underscore_single_alpha() {
        let expected = Ok(("", Identifier::new("_a")));
        let actual = identifier("_a");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_underscore_multi_alpha() {
        let expected = Ok(("", Identifier::new("_abcdef")));
        let actual = identifier("_abcdef");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_underscore_single_digit_zero() {
        let expected = Ok(("", Identifier::new("_0")));
        let actual = identifier("_0");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_underscore_single_digit() {
        let expected = Ok(("", Identifier::new("_1")));
        let actual = identifier("_1");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_underscore_multi_digit() {
        let expected = Ok(("", Identifier::new("_12345")));
        let actual = identifier("_12345");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multi_underscore() {
        let expected = Ok(("", Identifier::new("_____")));
        let actual = identifier("_____");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multi_underscore_multi_alpha() {
        let expected = Ok(("", Identifier::new("_a_b_c_d_e_f")));
        let actual = identifier("_a_b_c_d_e_f");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multi_underscore_multi_alpha_mulit_numeric() {
        let expected = Ok(("", Identifier::new("_a_2_3_4_e_5")));
        let actual = identifier("_a_2_3_4_e_5");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multi_underscore_multi_alpha_mulit_numeric_underscore() {
        let expected = Ok(("", Identifier::new("_a_2_3_4_e_5_")));
        let actual = identifier("_a_2_3_4_e_5_");
        assert_eq!(expected, actual);
    }
}
