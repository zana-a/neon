use crate::core::parser::result::Result;

pub fn left_bracket(input: &str) -> Result<&str, char> {
  nom::character::complete::char('[')(input)
}

pub fn right_bracket(input: &str) -> Result<&str, char> {
  nom::character::complete::char(']')(input)
}
