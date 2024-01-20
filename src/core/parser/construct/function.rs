//! # Function module
//!
//! A function is a unit of code. It is called by its name and respective parameters.
//!
//! Examples:
//!
//! Function with no parameters
//!
//! ```
//! def hello() do
//! end
//! ```
//! Function with single overloaded parameter
//!
//! ```
//! def hello(name: string) do
//! end
//! ```
//!
//! Function with two overloaded parameters
//!
//! ```
//! def hello(name: string, age: int) do
//! end
//! ```
//!
//! Function with a return type
//!
//! ```
//! def hello(name: string, age: int) -> string do
//! end
//! ```
//!
//! Calling the methods
//!
//! ```
//! hello()
//! hello("Zana")
//! hello("Zana", 24)
//! ```

// TODO: known issues of function parser: spacing on left side of arrow is fine but on right side it causes error.

use nom::character::complete::multispace1;
use nom::combinator::opt;
use nom::sequence::{delimited, preceded, tuple};

use crate::core::parser::primitive::block::{block, Block};
use crate::core::parser::primitive::identifier::{identifier, Identifier};
use crate::core::parser::primitive::parameters::{parameters, Parameters};
use crate::core::parser::result::Result;
use crate::core::parser::util::arrow::right_arrow;
use crate::core::parser::util::def::def;
use crate::core::parser::util::padded::padded0;
use crate::core::parser::util::parenthesis::{left_parenthesis, right_parenthesis};

#[derive(Debug, PartialEq)]
pub struct Function {
    name: Identifier,
    parameters: Option<Parameters>,
    returns: Option<Identifier>,
    block: Option<Block>,
}

fn function_name(input: &str) -> Result<&str, (Identifier, Parameters)> {
    tuple((
        identifier,
        delimited(left_parenthesis, parameters, right_parenthesis),
    ))(input)
    .map(|(remaining, result)| (remaining, result))
}

fn function_return(input: &str) -> Result<&str, Option<Identifier>> {
    opt(preceded(right_arrow, preceded(multispace1, identifier)))(input)
}

pub fn function(input: &str) -> Result<&str, Function> {
    tuple((
        preceded(tuple((def, multispace1)), function_name),
        tuple((padded0(function_return), block)),
    ))(input)
    .map(|(remaining, ((name, parameters), (returns, block)))| {
        let parameters = if parameters.parameters.is_empty() {
            None
        } else {
            Some(parameters)
        };

        let block = if block.body.is_none() {
            None
        } else {
            Some(block)
        };

        (
            remaining,
            Function {
                name,
                parameters,
                returns,
                block,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::core::parser::construct::expression::{Expression, ExpressionBody};
    use crate::core::parser::primitive::boolean::Boolean;
    use crate::core::parser::primitive::expressions::Expressions;
    use crate::core::parser::primitive::pair::Pair;

    use super::*;

    #[test]
    fn empty_block() {
        let expected = Ok((
            "",
            Function {
                name: Identifier {
                    value: String::from("hello"),
                },
                parameters: None,
                returns: None,
                block: None,
            },
        ));
        let actual = function("def hello() do end");
        assert_eq!(expected, actual)
    }

    #[test]
    fn expression_block() {
        let expected = Ok((
            "",
            Function {
                name: Identifier {
                    value: String::from("hello"),
                },
                parameters: None,
                returns: None,
                block: Some(Block {
                    body: Some(Expressions {
                        expressions: vec![Expression {
                            body: ExpressionBody::Boolean(Boolean::True),
                        }],
                    }),
                }),
            },
        ));
        let actual = function("def hello() do true end");
        assert_eq!(expected, actual)
    }

    #[test]
    fn parameter_block() {
        let expected = Ok((
            "",
            Function {
                name: Identifier {
                    value: String::from("hello"),
                },
                parameters: Some(Parameters {
                    parameters: vec![Pair {
                        identifier: Identifier {
                            value: String::from("name"),
                        },
                        kind: Identifier {
                            value: String::from("string"),
                        },
                    }],
                }),
                returns: None,
                block: Some(Block {
                    body: Some(Expressions {
                        expressions: vec![Expression {
                            body: ExpressionBody::Boolean(Boolean::True),
                        }],
                    }),
                }),
            },
        ));

        let actual = function("def hello(name: string) do true end");
        assert_eq!(expected, actual);

        let expected = Ok((
            "",
            Function {
                name: Identifier {
                    value: String::from("hello"),
                },
                parameters: Some(Parameters {
                    parameters: vec![
                        Pair {
                            identifier: Identifier {
                                value: String::from("name"),
                            },
                            kind: Identifier {
                                value: String::from("string"),
                            },
                        },
                        Pair {
                            identifier: Identifier {
                                value: String::from("age"),
                            },
                            kind: Identifier {
                                value: String::from("int"),
                            },
                        },
                    ],
                }),
                returns: None,
                block: Some(Block {
                    body: Some(Expressions {
                        expressions: vec![Expression {
                            body: ExpressionBody::Boolean(Boolean::True),
                        }],
                    }),
                }),
            },
        ));

        let actual = function("def hello(name: string, age: int) do true end");
        assert_eq!(expected, actual)
    }

    #[test]
    fn block_with_return() {
        let expected = Ok((
            "",
            Function {
                name: Identifier {
                    value: String::from("hello"),
                },
                parameters: None,
                returns: Some(Identifier {
                    value: String::from("string"),
                }),
                block: None,
            },
        ));
        let actual = function("def hello() -> string do end");
        assert_eq!(expected, actual)
    }
}
