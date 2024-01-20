//! # Parameters module
//!
//! A parameter is a comma seperated list of [Pair]
//!
//! Examples:
//!
//! Padded:
//!
//! ```
//! a: 1, b: 2, c: 3
//! ```
//!
//! Not padded
//!
//! ```
//! a:1,b:2,c:3
//! ```

use nom::multi::separated_list0;

use crate::core::parser::primitive::pair::{pair, Pair};
use crate::core::parser::result::Result;
use crate::core::parser::util::comma::comma;
use crate::core::parser::util::padded::padded0;

#[derive(Debug, PartialEq)]
pub struct Parameters {
    pub parameters: Vec<Pair>,
}

pub fn parameters(input: &str) -> Result<&str, Parameters> {
    separated_list0(padded0(comma), pair)(input)
        .map(|(remaining, parameters)| (remaining, Parameters { parameters }))
}

#[cfg(test)]
mod tests {
    use crate::core::parser::primitive::identifier::Identifier;

    use super::*;

    #[test]
    fn empty() {
        let expected = Ok(("", Parameters { parameters: vec![] }));
        let result = parameters("");
        assert_eq!(expected, result)
    }

    #[test]
    fn single() {
        let expected = Ok((
            "",
            Parameters {
                parameters: vec![Pair {
                    identifier: Identifier {
                        value: "a".to_string(),
                    },
                    kind: Identifier {
                        value: "a".to_string(),
                    },
                }],
            },
        ));
        let result = parameters("a: a");
        assert_eq!(expected, result);
    }

    #[test]
    fn multiple() {
        let expected = Ok((
            "",
            Parameters {
                parameters: vec![
                    Pair {
                        identifier: Identifier {
                            value: "a".to_string(),
                        },
                        kind: Identifier {
                            value: "a".to_string(),
                        },
                    },
                    Pair {
                        identifier: Identifier {
                            value: "b".to_string(),
                        },
                        kind: Identifier {
                            value: "b".to_string(),
                        },
                    },
                    Pair {
                        identifier: Identifier {
                            value: "c".to_string(),
                        },
                        kind: Identifier {
                            value: "c".to_string(),
                        },
                    },
                ],
            },
        ));
        let result = parameters("a:a,b:b,c:c");
        assert_eq!(expected, result);

        let result = parameters("a: a \n\t,\n\t b: b \n\t,\n\t c: c");
        assert_eq!(expected, result);
    }
}
