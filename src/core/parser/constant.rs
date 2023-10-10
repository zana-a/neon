use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    sequence::{preceded, tuple},
    IResult,
};

use super::{
    boolean::{boolean, Boolean},
    equals::equals,
    identifier::Identifier,
    padded::padded0,
    pair::pair,
};

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

    use crate::core::parser::boolean::Boolean;

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
