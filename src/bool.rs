use nom::branch::alt;
use nom::bytes::complete::tag;

use crate::prelude::parser;

#[derive(PartialEq, Debug)]
pub enum Bool {
    True,
    False,
}

fn _true(s: &str) -> parser::Result<&str, Bool> {
    tag("true")(s).map(|(remaining, _)| (remaining, Bool::True))
}

fn _false(s: &str) -> parser::Result<&str, Bool> {
    tag("false")(s).map(|(remaining, _)| (remaining, Bool::False))
}

pub fn bool(s: &str) -> parser::Result<&str, Bool> {
    alt((_true, _false))(s)
}

#[cfg(test)]
mod tests {
    use crate::bool::bool;
    use crate::bool::Bool;

    #[test]
    fn test_true() {
        let expected = Ok(("", Bool::True));
        let actual = bool("true");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_false() {
        let expected = Ok(("", Bool::False));
        let actual = bool("false");
        assert_eq!(expected, actual);
    }
}
