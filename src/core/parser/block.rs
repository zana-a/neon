use nom::{combinator::opt, multi::many1, sequence::delimited, IResult};

use super::{
    brace::{left_brace, right_brace},
    expression::{expression, Expression},
    padded::padded0,
};

#[derive(Debug, PartialEq)]
pub struct Block {
    body: Option<Vec<Expression>>,
}

pub fn block(input: &str) -> IResult<&str, Block> {
    delimited(
        left_brace,
        padded0(opt(many1(padded0(expression)))),
        right_brace,
    )(input)
    .map(|(remaining, result)| (remaining, Block { body: result }))
}

#[cfg(test)]
mod tests {

    use crate::core::parser::boolean::Boolean;

    use super::*;

    #[test]
    fn empty_block() {
        let expected = Ok(("", Block { body: None }));

        let result = block("{}");
        assert_eq!(expected, result);

        let result = block("{   }");
        assert_eq!(expected, result);

        let result = block("{\t\t\t}");
        assert_eq!(expected, result);

        let result = block("{\n\n\n}");
        assert_eq!(expected, result);
    }

    #[test]
    fn single_expression_block() {
        let expected = Ok((
            "",
            Block {
                body: Some(vec![Expression::Boolean(Boolean::True)]),
            },
        ));

        let result = block("{true}");
        assert_eq!(expected, result);

        let result = block("{   true   }");
        assert_eq!(expected, result);

        let result = block("{\t\t\ttrue\t\t\t}");
        assert_eq!(expected, result);

        let result = block("{\n\n\ntrue\n\n\n}");
        assert_eq!(expected, result);
    }

    #[test]
    fn multi_expression_block() {
        let expected = Ok((
            "",
            Block {
                body: Some(vec![
                    Expression::Boolean(Boolean::True),
                    Expression::Boolean(Boolean::False),
                    Expression::Boolean(Boolean::True),
                ]),
            },
        ));

        let result = block("{true false true}");
        assert_eq!(expected, result);

        let result = block("{true   false   true}");
        assert_eq!(expected, result);

        let result = block("{true\t\t\tfalse\t\t\ttrue}");
        assert_eq!(expected, result);
    }
}
