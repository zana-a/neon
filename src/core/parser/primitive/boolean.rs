//! # Boolean module
//!
//! A boolean can be either true or false.
//!
//! Reference:
//! ```
//! <true> or <false>
//! ```
//!
//! Example:
//! ```
//! true
//! false
//! ```

use nom::{branch::alt, bytes::complete::tag, IResult};

#[derive(PartialEq, Debug)]
pub enum Boolean {
    True,
    False,
}

fn t(input: &str) -> IResult<&str, Boolean> {
    tag("true")(input).map(|(remaining, _)| (remaining, Boolean::True))
}

fn f(input: &str) -> IResult<&str, Boolean> {
    tag("false")(input).map(|(remaining, _)| (remaining, Boolean::False))
}

pub fn boolean(input: &str) -> IResult<&str, Boolean> {
    alt((t, f))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() {
        let expected = Ok(("", Boolean::True));
        let actual = boolean("true");
        assert_eq!(expected, actual);
    }

    #[test]
    fn f() {
        let expected = Ok(("", Boolean::False));
        let actual = boolean("false");
        assert_eq!(expected, actual);
    }
}
