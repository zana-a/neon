mod structure;

use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::all_consuming;
use nom::sequence::delimited;
use nom::sequence::separated_pair;

pub use crate::core::parser::variable::structure::Variable;

use crate::core::parser::bool::bool;
use crate::core::parser::identifier::identifier;
use crate::core::parser::prelude::Result;

fn equals(input: &str) -> Result<&str, &str> {
    delimited(multispace0, tag("="), multispace0)(input)
}

pub fn variable(input: &str) -> Result<&str, Variable> {
    all_consuming(separated_pair(identifier, equals, bool))(input)
        .map(|(remaining, (identifier, value))| (remaining, Variable { identifier, value }))
}

#[cfg(test)]
mod tests {

    use crate::core::parser::bool::*;
    use crate::core::parser::identifier::*;
    use crate::core::parser::variable::*;

    #[test]
    fn unpadded() {
        let expected = Ok((
            "",
            Variable {
                identifier: Identifier {
                    value: String::from("a"),
                },
                value: Bool::True,
            },
        ));
        let actual = variable("a=true");
        assert_eq!(expected, actual);
    }

    #[test]
    fn padded() {
        let expected = Ok((
            "",
            Variable {
                identifier: Identifier {
                    value: String::from("a"),
                },
                value: Bool::True,
            },
        ));
        let actual = variable("a = true");
        assert_eq!(expected, actual);
    }
}
