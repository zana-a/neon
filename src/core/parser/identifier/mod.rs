mod structure;

use nom::branch::alt;
use nom::combinator::*;
use nom::multi::many1;
use nom::sequence::pair;

pub use crate::core::parser::identifier::structure::Identifier;

use crate::core::parser::prelude::*;

pub fn identifier(input: &str) -> Result<&str, Identifier> {
    recognize(pair(
        alt((alpha, underscore)),
        opt(many1(alt((alpha, numeric, underscore)))),
    ))(input)
    .map(|(remaining, value)| {
        (
            remaining,
            Identifier {
                value: String::from(value),
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::core::parser::identifier::*;

    #[test]
    fn underscore() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_"),
            },
        ));
        let actual = identifier("_");
        assert_eq!(expected, actual);
    }

    #[test]
    fn single_alpha() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("a"),
            },
        ));
        let actual = identifier("a");
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_alpha() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("abcdefg"),
            },
        ));
        let actual = identifier("abcdefg");
        assert_eq!(expected, actual);
    }

    #[test]
    fn underscore_single_alpha() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_a"),
            },
        ));
        let actual = identifier("_a");
        assert_eq!(expected, actual);
    }

    #[test]
    fn underscore_multi_alpha() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_abcdef"),
            },
        ));
        let actual = identifier("_abcdef");
        assert_eq!(expected, actual);
    }

    #[test]
    fn underscore_single_digit_zero() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_0"),
            },
        ));
        let actual = identifier("_0");
        assert_eq!(expected, actual);
    }

    #[test]
    fn underscore_single_digit() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_1"),
            },
        ));
        let actual = identifier("_1");
        assert_eq!(expected, actual);
    }

    #[test]
    fn underscore_multi_digit() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_12345"),
            },
        ));
        let actual = identifier("_12345");
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_underscore() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_____"),
            },
        ));
        let actual = identifier("_____");
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_underscore_multi_alpha() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_a_b_c_d_e_f"),
            },
        ));
        let actual = identifier("_a_b_c_d_e_f");
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_underscore_multi_alpha_mulit_numeric() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_a_2_3_4_e_5"),
            },
        ));
        let actual = identifier("_a_2_3_4_e_5");
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_underscore_multi_alpha_mulit_numeric_underscore() {
        let expected = Ok((
            "",
            Identifier {
                value: String::from("_a_2_3_4_e_5_"),
            },
        ));
        let actual = identifier("_a_2_3_4_e_5_");
        assert_eq!(expected, actual);
    }
}
