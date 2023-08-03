use crate::core::parser::bool::Bool;
use crate::core::parser::identifier::Identifier;

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub identifier: Identifier,
    pub value: Bool,
}
