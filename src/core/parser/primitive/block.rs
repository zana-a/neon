use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::sequence::delimited;

use crate::core::parser::primitive::expressions::expressions;
use crate::core::parser::primitive::expressions::Expressions;
use crate::core::parser::result::Result;
use crate::core::parser::util::end::end;
use crate::core::parser::util::padded::padded1;
use crate::core::parser::util::r#do::r#do;

// TODO: needs support for statements
#[derive(Debug, PartialEq)]
pub struct Block {
  pub body: Option<Expressions>,
}

fn populated_block(input: &str) -> Result<&str, Block> {
  delimited(r#do, padded1(expressions), end)(input).map(
    |(remaining, expressions)| {
      let body = if expressions.is_empty() {
        None
      } else {
        Some(Expressions { expressions })
      };

      (remaining, Block { body })
    },
  )
}

fn empty_block(input: &str) -> Result<&str, Block> {
  delimited(r#do, multispace1, end)(input)
    .map(|(remaining, _)| (remaining, Block { body: None }))
}

pub fn block(input: &str) -> Result<&str, Block> {
  alt((populated_block, empty_block))(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::parser::construct::expression::Expression;
  use crate::core::parser::construct::expression::ExpressionBody;
  use crate::core::parser::primitive::boolean::Boolean;

  #[test]
  fn empty_block() {
    let expected = Ok(("", Block { body: None }));
    let result = block("do end");
    assert_eq!(expected, result);
  }

  #[test]
  fn expression_block() {
    let expected = Ok((
      "",
      Block {
        body: Some(Expressions {
          expressions: vec![Expression {
            body: ExpressionBody::Boolean(Boolean::True),
          }],
        }),
      },
    ));
    let result = block("do true end");
    assert_eq!(expected, result);
  }
}
