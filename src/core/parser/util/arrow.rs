use nom::bytes::complete::tag;

use crate::core::parser::result::Result;

pub fn right_arrow(input: &str) -> Result<&str, &str> {
  tag("->")(input)
}
