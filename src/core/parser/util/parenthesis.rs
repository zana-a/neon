use crate::core::parser::result::Result;

pub fn left_parenthesis(input: &str) -> Result<&str, char> {
    nom::character::complete::char('(')(input)
}

pub fn right_parenthesis(input: &str) -> Result<&str, char> {
    nom::character::complete::char(')')(input)
}
