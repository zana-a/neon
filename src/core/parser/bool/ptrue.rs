use nom::bytes::complete::tag;

use crate::core::parser::bool::structure::*;
use crate::core::parser::prelude::*;

pub fn ptrue(input: &str) -> Result<&str, Bool> {
    tag("true")(input).map(|(remaining, _)| (remaining, Bool::True))
}
