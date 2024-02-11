use nom::character::complete::anychar;
use nom::combinator::verify;

use crate::core::parser::result::Result;

pub fn numeric(input: &str) -> Result<&str, char> {
  verify(anychar, |&c| c.is_numeric())(input)
}
