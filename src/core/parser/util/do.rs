use nom::bytes::complete::tag;

use crate::core::parser::result::Result;

pub fn r#do(input: &str) -> Result<&str, &str> {
    tag("do")(input)
}
