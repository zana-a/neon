use crate::core::parser::result::Result;
use nom::bytes::complete::tag;

pub fn def(input: &str) -> Result<&str, &str> {
    tag("def")(input)
}
