mod pfalse;
mod ptrue;
mod structure;

use nom::branch::alt;

use crate::core::parser::bool::pfalse::*;
use crate::core::parser::bool::ptrue::*;
use crate::core::parser::bool::structure::*;
use crate::core::parser::prelude::*;

pub fn bool(input: &str) -> Result<&str, Bool> {
    alt((ptrue, pfalse))(input)
}

#[cfg(test)]
mod tests {
    use crate::core::parser::bool::*;

    #[test]
    fn t_true() {
        let expected = Ok(("", Bool::True));
        let actual = bool("true");
        assert_eq!(expected, actual);
    }

    #[test]
    fn t_false() {
        let expected = Ok(("", Bool::False));
        let actual = bool("false");
        assert_eq!(expected, actual);
    }
}
