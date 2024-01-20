use crate::core::parser::result::Result;

pub fn equals(input: &str) -> Result<&str, char> {
    nom::character::complete::char('=')(input)
}
