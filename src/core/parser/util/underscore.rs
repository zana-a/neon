use crate::core::parser::result::Result;

pub fn underscore(input: &str) -> Result<&str, char> {
    nom::character::complete::char('_')(input)
}
