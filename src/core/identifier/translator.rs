use crate::core::identifier::parser::Identifier;

pub fn translate(identifier: &Identifier) -> String {
    format!("{:?}", identifier)
}
