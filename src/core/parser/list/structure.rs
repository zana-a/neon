use crate::core::parser::bool::Bool;

#[derive(Debug, PartialEq)]
pub struct List {
    pub values: Vec<Bool>,
}
