use nom::bytes::complete::tag;

use crate::prelude::result::*;

pub fn parse(s: &str) -> ParseResult<&str, &str> {
    tag("s")(s)
}
