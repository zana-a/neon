use nom::bytes::complete::tag;

use crate::core::parser::result::Result;

pub fn def(input: &str) -> Result<&str, &str> {
  tag("def")(input)
}
