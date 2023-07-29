mod result;

use nom::character::complete::anychar;
use nom::character::complete::char;
use nom::combinator::verify;

pub use crate::core::parser::prelude::result::*;

pub fn alpha(input: &str) -> Result<&str, char> {
    verify(anychar, |&c| c.is_alphabetic())(input)
}

pub fn numeric(input: &str) -> Result<&str, char> {
    verify(anychar, |&c| c.is_numeric())(input)
}

pub fn underscore(input: &str) -> Result<&str, char> {
    char('_')(input)
}
