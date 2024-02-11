use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::sequence::preceded;
use nom::sequence::tuple;

use crate::core::parser::primitive::boolean::boolean;
use crate::core::parser::primitive::boolean::Boolean;
use crate::core::parser::primitive::identifier::Identifier;
use crate::core::parser::primitive::pair::pair;
use crate::core::parser::result::Result;
use crate::core::parser::util::equals::equals;
use crate::core::parser::util::padded::padded0;

#[derive(Debug, PartialEq)]
pub struct Constant {
  pub identifier: Identifier,
  pub kind: Identifier,
  pub body: Boolean,
}

pub fn constant(input: &str) -> Result<&str, Constant> {
  let declaration = preceded(tag("let"), preceded(multispace1, pair));
  let assigment = preceded(padded0(equals), boolean);

  tuple((declaration, assigment))(input).map(|(remaining, (pair, body))| {
    (
      remaining,
      Constant {
        identifier: pair.identifier,
        kind: pair.kind,
        body,
      },
    )
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_variable() {
    let expected = Ok((
      "",
      Constant {
        identifier: Identifier {
          value: String::from("a"),
        },
        kind: Identifier {
          value: String::from("bool"),
        },
        body: Boolean::True,
      },
    ));

    let result = constant("let a: bool = true");
    assert_eq!(expected, result);

    let result = constant("let a:bool=true");
    assert_eq!(expected, result);
  }
}
