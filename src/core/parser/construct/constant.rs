//! # Constant module
//!
//! A constant is a construct that represents a value that cannot be changed. In addition,
//! the constant can be called using its identifier.
//!
//! Reference:
//! ```
//! let <identifier>: <type> = <expression>
//! ```
//!
//! Example:
//! ```
//! let my_number: int = 23
//! ```

use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::core::parser::primitive::boolean::{boolean, Boolean};
use crate::core::parser::primitive::identifier::Identifier;
use crate::core::parser::primitive::pair::pair;
use crate::core::parser::util::equals::equals;
use crate::core::parser::util::padded::padded0;

#[derive(Debug, PartialEq)]
pub struct Constant {
    pub identifier: Identifier,
    pub kind: Identifier,
    pub body: Boolean,
}

pub fn constant(input: &str) -> IResult<&str, Constant> {
    let declaration = preceded(tag("let"), preceded(multispace1, pair));
    let assigment = preceded(padded0(equals), boolean);

    tuple((declaration, assigment))(input).map(|(remaining, (pair, body))| {
        (
            remaining,
            Constant {
                identifier: pair.identifier,
                kind: pair.kind,
                body,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable() {
        let expected = Ok((
            "",
            Constant {
                identifier: Identifier {
                    value: String::from("a"),
                },
                kind: Identifier {
                    value: String::from("bool"),
                },
                body: Boolean::True,
            },
        ));

        let result = constant("let a: bool = true");
        assert_eq!(expected, result);

        let result = constant("let a:bool=true");
        assert_eq!(expected, result);
    }
}
