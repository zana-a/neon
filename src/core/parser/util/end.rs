use nom::bytes::complete::tag;

use crate::core::parser::result::Result;

pub fn end(input: &str) -> Result<&str, &str> {
    tag("end")(input)
}
