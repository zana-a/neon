use crate::core::parser::result::Result;
use nom::{character::complete::anychar, combinator::verify};

pub fn numeric(input: &str) -> Result<&str, char> {
    verify(anychar, |&c| c.is_numeric())(input)
}
