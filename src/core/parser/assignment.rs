use std::result;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    multi,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use super::{
    boolean::Boolean,
    equals::{self, equals},
    expression::{expression, Expression},
    identifier::{identifier, Identifier},
    padded::{padded0, padded1},
    pair::{self, pair},
    semi_colon::semi_colon,
};

#[derive(Debug, PartialEq)]
pub struct AssignmentBody {
    identifier: Identifier,
    kind: Identifier,
    body: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Assignment {
    Variable(AssignmentBody),
    Constant(AssignmentBody),
}

fn variable(input: &str) -> IResult<&str, Assignment> {
    let var_declaration = preceded(tag("var"), preceded(multispace1, pair));
    let var_assigment = preceded(padded0(equals), expression);

    tuple((var_declaration, var_assigment))(input).map(|(remaining, (pair, body))| {
        (
            remaining,
            Assignment::Variable(AssignmentBody {
                identifier: pair.identifier,
                kind: pair.kind,
                body,
            }),
        )
    })
}

fn constant(input: &str) -> IResult<&str, Assignment> {
    let var_declaration = preceded(tag("val"), preceded(multispace1, pair));
    let var_assigment = preceded(padded0(equals), expression);

    tuple((var_declaration, var_assigment))(input).map(|(remaining, (pair, body))| {
        (
            remaining,
            Assignment::Constant(AssignmentBody {
                identifier: pair.identifier,
                kind: pair.kind,
                body,
            }),
        )
    })
}

pub fn assignment(input: &str) -> IResult<&str, Assignment> {
    alt((variable, constant))(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_variable() {
        let expected = Ok((
            "",
            Assignment::Variable(AssignmentBody {
                identifier: Identifier {
                    value: String::from("a"),
                },
                kind: Identifier {
                    value: String::from("Boolean"),
                },
                body: Expression::Boolean(Boolean::True),
            }),
        ));

        let result = assignment("var a: Boolean = true;");
        assert_eq!(expected, result);

        let result = assignment("var a:Boolean=true;");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_constant() {
        let expected = Ok((
            "",
            Assignment::Constant(AssignmentBody {
                identifier: Identifier {
                    value: String::from("a"),
                },
                kind: Identifier {
                    value: String::from("Boolean"),
                },
                body: Expression::Boolean(Boolean::True),
            }),
        ));

        let result = assignment("val a: Boolean = true;");
        assert_eq!(expected, result);

        let result = assignment("val a:Boolean=true;");
        assert_eq!(expected, result);
    }
}
