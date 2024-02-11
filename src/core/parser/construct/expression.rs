use nom::branch::alt;

use crate::core::parser::primitive::boolean::boolean;
use crate::core::parser::primitive::boolean::Boolean;
use crate::core::parser::primitive::integer::integer;
use crate::core::parser::primitive::integer::Integer;
use crate::core::parser::result::Result;

#[derive(Debug, PartialEq)]
pub enum ExpressionBody {
  // TODO: generalise this
  // 1. Integer -> 1
  // 2. Float -> 1.2
  // 3. Arithmetic -> 1 + 2 / 2 - 3
  Arithmetic(Integer),
  Boolean(Boolean),
}

#[derive(Debug, PartialEq)]
pub struct Expression {
  pub body: ExpressionBody,
}

fn expressed_integer(input: &str) -> Result<&str, ExpressionBody> {
  integer(input).map(|(remaining, integer)| {
    (remaining, ExpressionBody::Arithmetic(integer))
  })
}

fn expressed_boolean(input: &str) -> Result<&str, ExpressionBody> {
  boolean(input)
    .map(|(remaining, boolean)| (remaining, ExpressionBody::Boolean(boolean)))
}

pub fn expression(input: &str) -> Result<&str, Expression> {
  alt((expressed_integer, expressed_boolean))(input)
    .map(|(remaining, body)| (remaining, Expression { body }))
}

#[cfg(test)]
mod tests {
  use crate::core::parser::construct::expression::expression;
  use crate::core::parser::construct::expression::Expression;
  use crate::core::parser::construct::expression::ExpressionBody;
  use crate::core::parser::primitive::boolean::Boolean;
  use crate::core::parser::primitive::integer::Integer;

  #[test]
  fn expressed_integer() {
    let expected = Ok((
      "",
      Expression {
        body: ExpressionBody::Arithmetic(Integer(24)),
      },
    ));
    let result = expression("24");
    assert_eq!(expected, result);
  }

  #[test]
  fn expressed_boolean() {
    let expected = Ok((
      "",
      Expression {
        body: ExpressionBody::Boolean(Boolean::True),
      },
    ));
    let result = expression("true");
    assert_eq!(expected, result);
  }
}
