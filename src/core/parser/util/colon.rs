use crate::core::parser::result::Result;

pub fn colon(input: &str) -> Result<&str, char> {
  nom::character::complete::char(':')(input)
}
