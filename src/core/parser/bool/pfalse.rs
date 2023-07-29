use nom::bytes::complete::tag;

use crate::core::parser::bool::structure::*;
use crate::core::parser::prelude::*;

pub fn pfalse(input: &str) -> Result<&str, Bool> {
    tag("false")(input).map(|(remaining, _)| (remaining, Bool::False))
}
