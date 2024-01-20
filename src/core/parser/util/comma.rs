use crate::core::parser::result::Result;

pub fn comma(input: &str) -> Result<&str, char> {
    nom::character::complete::char(',')(input)
}
