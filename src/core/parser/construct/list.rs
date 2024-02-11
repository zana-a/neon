use nom::multi::separated_list0;
use nom::sequence::delimited;

use crate::core::parser::primitive::boolean::boolean;
use crate::core::parser::primitive::boolean::Boolean;
use crate::core::parser::result::Result;
use crate::core::parser::util::bracket::left_bracket;
use crate::core::parser::util::bracket::right_bracket;
use crate::core::parser::util::comma::comma;
use crate::core::parser::util::padded::padded0;

#[derive(Debug, PartialEq)]
pub struct List {
  pub body: Option<Vec<Boolean>>,
}

pub fn list(input: &str) -> Result<&str, List> {
  delimited(
    left_bracket,
    padded0(separated_list0(comma, padded0(boolean))),
    right_bracket,
  )(input)
  .map(|(remaining, result)| {
    if remaining.is_empty() {
      (remaining, List { body: None })
    } else {
      (remaining, List { body: Some(result) })
    }
  })
}

#[cfg(test)]
mod tests {
  use crate::core::parser::construct::list::list;
  use crate::core::parser::construct::list::List;

  #[test]
  fn empty() {
    let expected = list("[]");
    let actual = Ok(("", List { body: None }));
    assert_eq!(expected, actual);

    let expected = list("[\n\t ]");
    let actual = Ok(("", List { body: None }));
    assert_eq!(expected, actual);
  }

  #[test]
  fn single() {
    let expected = list("[true]");
    let actual = Ok(("", List { body: None }));
    assert_eq!(expected, actual);

    let expected = list("[ true ]");
    let actual = Ok(("", List { body: None }));
    assert_eq!(expected, actual);
  }

  #[test]
  fn many() {
    let expected = list("[true,false,true,false]");
    let actual = Ok(("", List { body: None }));
    assert_eq!(expected, actual);

    let expected = list("[ true , false, true ,false]");
    let actual = Ok(("", List { body: None }));
    assert_eq!(expected, actual);
  }
}
