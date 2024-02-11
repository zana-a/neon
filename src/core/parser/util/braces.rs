use crate::core::parser::result::Result;

pub fn left_brace(input: &str) -> Result<&str, char> {
  nom::character::complete::char('{')(input)
}

pub fn right_brace(input: &str) -> Result<&str, char> {
  nom::character::complete::char('}')(input)
}
