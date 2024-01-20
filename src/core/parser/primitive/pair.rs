// todo - remove this module and replace it with less generic version

//! # Pair module
//!
//! A pair represents a key and value seperated by a colon.
//! This module is primitive and can only hold identifiers as both key and value.
//! You can recompose it onto a more specific Pair struct.
//!
//! Reference:
//! ```
//! <identifier>: <identifier>
//! ```
//!
//! Example:
//! ```
//! a: b
//! _hello: world
//! ```

use nom::sequence::separated_pair;

use crate::core::parser::primitive::identifier::{identifier, Identifier};
use crate::core::parser::result::Result;
use crate::core::parser::util::colon::colon;
use crate::core::parser::util::padded::padded0;

#[derive(Debug, PartialEq)]
pub struct Pair {
    pub identifier: Identifier,
    pub kind: Identifier,
}

pub fn pair(input: &str) -> Result<&str, Pair> {
    separated_pair(identifier, padded0(colon), identifier)(input).map(|(remaining, result)| {
        (
            remaining,
            Pair {
                identifier: result.0,
                kind: result.1,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expected = Ok((
            "",
            Pair {
                identifier: Identifier {
                    value: String::from("a"),
                },
                kind: Identifier {
                    value: String::from("a"),
                },
            },
        ));

        let result = pair("a:a");
        assert_eq!(expected, result);

        let result = pair("a   :a");
        assert_eq!(expected, result);

        let result = pair("a:   a");
        assert_eq!(expected, result);

        let result = pair("a\t\t\t:a");
        assert_eq!(expected, result);

        let result = pair("a:\t\t\ta");
        assert_eq!(expected, result);

        let result = pair("a\n\n\n:a");
        assert_eq!(expected, result);

        let result = pair("a:\n\n\na");
        assert_eq!(expected, result);
    }
}
