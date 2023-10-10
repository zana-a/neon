use nom::{branch::alt, IResult};

use super::{boolean::Boolean, constant::Constant};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Boolean(Boolean),
    Constant(Constant),
}

fn boolean(input: &str) -> IResult<&str, Expression> {
    super::boolean::boolean(input)
        .map(|(remaining, boolean)| (remaining, Expression::Boolean(boolean)))
}

fn constant(input: &str) -> IResult<&str, Expression> {
    super::constant::constant(input)
        .map(|(remaining, constant)| (remaining, Expression::Constant(constant)))
}

// TODO: add alt parser to check for different viable expression
// TODO: change boolean to primitive parser (which includes boolean and other primitives)
pub fn expression(input: &str) -> IResult<&str, Expression> {
    alt((boolean, constant))(input)
}

#[cfg(test)]
mod tests {

    use crate::core::parser::identifier::Identifier;

    use super::*;

    #[test]
    fn boolean_expression() {
        let expected = Ok(("", Expression::Boolean(Boolean::True)));
        let result = expression("true");
        assert_eq!(expected, result);
    }

    #[test]
    fn constant_expression() {
        let expected = Ok((
            "",
            Expression::Constant(Constant {
                identifier: Identifier {
                    value: String::from("a"),
                },
                kind: Identifier {
                    value: String::from("bool"),
                },
                body: Boolean::True,
            }),
        ));
        let result = expression("let a: bool = true");
        assert_eq!(expected, result);
    }
}
