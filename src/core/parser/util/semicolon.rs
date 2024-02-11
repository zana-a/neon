use crate::core::parser::result::Result;

pub fn semi_colon(input: &str) -> Result<&str, char> {
  nom::character::complete::char(';')(input)
}
