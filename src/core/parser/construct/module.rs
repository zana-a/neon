//! # Module module
//!
//! A module can be used to package likewise functions and modules together.
//!
//! Examples:
//!
//! Empty module
//!
//! ```
//! module A do
//! end
//! ```
//!
//! Nested modules
//!
//! ```
//! module A do
//!   module B do
//!     module C do
//!     end
//!   end
//! end
//! ```
//!
//! Multi-modules
//!
//! ```
//! module A do
//!   module B do
//!   end
//!
//!   module C do
//!   end
//! end
//! ```
//!
//! Functions
//!
//! ```
//! module A do
//!   def foo() do
//!   end
//!
//!   def bar() do
//!   end
//!
//!   def baz() do
//!   end
//! end
//! ```
//!

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::core::parser::construct::function::{function, Function};
use crate::core::parser::primitive::identifier::{identifier, Identifier};
use crate::core::parser::util::end::end;
use crate::core::parser::util::padded::padded1;
use crate::core::parser::util::r#do::r#do;

#[derive(Debug, PartialEq)]
pub enum ModuleBody {
    Function(Function),
    Module(Box<Module>),
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub identifier: Identifier,
    pub body: Option<Vec<ModuleBody>>,
}

fn modularised_function(input: &str) -> IResult<&str, ModuleBody> {
    function(input).map(|(remaining, function)| (remaining, ModuleBody::Function(function)))
}

fn modularised_module(input: &str) -> IResult<&str, ModuleBody> {
    module(input).map(|(remaining, module)| (remaining, ModuleBody::Module(Box::new(module))))
}

fn populated_block(input: &str) -> IResult<&str, Option<Vec<ModuleBody>>> {
    let module_or_function = alt((modularised_module, modularised_function));
    delimited(r#do, many1(padded1(module_or_function)), end)(input)
        .map(|(remaining, result)| (remaining, Some(result)))
}

fn empty_block(input: &str) -> IResult<&str, Option<Vec<ModuleBody>>> {
    terminated(preceded(r#do, multispace1), end)(input).map(|(remaining, _)| (remaining, None))
}

fn block(input: &str) -> IResult<&str, Option<Vec<ModuleBody>>> {
    alt((populated_block, empty_block))(input)
}

fn module_identifier(input: &str) -> IResult<&str, Identifier> {
    preceded(tag("module"), padded1(identifier))(input)
}

pub fn module(input: &str) -> IResult<&str, Module> {
    tuple((module_identifier, block))(input)
        .map(|(remaining, (identifier, body))| (remaining, Module { identifier, body }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_module() {
        let expected = Ok((
            "",
            Module {
                identifier: Identifier {
                    value: String::from("A"),
                },
                body: None,
            },
        ));
        let actual = module("module A do end");
        assert_eq!(expected, actual);
    }

    #[test]
    fn nested_modules() {
        let expected = Ok((
            "",
            Module {
                identifier: Identifier {
                    value: String::from("A"),
                },
                body: Some(vec![ModuleBody::Module(Box::new(Module {
                    identifier: Identifier {
                        value: String::from("B"),
                    },
                    body: Some(vec![ModuleBody::Module(Box::new(Module {
                        identifier: Identifier {
                            value: String::from("C"),
                        },
                        body: None,
                    }))]),
                }))]),
            },
        ));
        let actual = module(
            r#"
        module A do
          module B do
            module C do
            end
          end
        end
        "#
            .trim(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_module() {
        let expected = Ok((
            "",
            Module {
                identifier: Identifier {
                    value: String::from("A"),
                },
                body: Some(vec![
                    ModuleBody::Module(Box::new(Module {
                        identifier: Identifier {
                            value: String::from("B"),
                        },
                        body: None,
                    })),
                    ModuleBody::Module(Box::new(Module {
                        identifier: Identifier {
                            value: String::from("C"),
                        },
                        body: None,
                    })),
                ]),
            },
        ));
        let actual = module(
            r#"
        module A do
          module B do
          end
          module C do
          end
        end
        "#
            .trim(),
        );
        assert_eq!(expected, actual);
    }
}
