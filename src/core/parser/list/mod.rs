mod structure;

use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::all_consuming;
use nom::multi::separated_list0;
use nom::sequence::delimited;

pub use crate::core::parser::list::structure::List;

use crate::core::parser::bool::*;
use crate::core::parser::prelude::Result;

fn items(input: &str) -> Result<&str, Vec<Bool>> {
    separated_list0(delimited(multispace0, tag(","), multispace0), bool)(input)
}

pub fn list(input: &str) -> Result<&str, List> {
    all_consuming(delimited(
        tag("["),
        delimited(multispace0, items, multispace0),
        tag("]"),
    ))(input)
    .map(|(remaining, values)| (remaining, List { values }))
}

#[cfg(test)]
mod tests {

    use crate::core::parser::bool::Bool;
    use crate::core::parser::list::*;

    #[test]
    fn empty() {
        let expected = Ok(("", List { values: vec![] }));
        let actual = list("[]");
        assert_eq!(expected, actual);
    }

    #[test]
    fn single() {
        let expected = Ok((
            "",
            List {
                values: vec![Bool::True],
            },
        ));
        let actual = list("[true]");
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi() {
        let expected = Ok((
            "",
            List {
                values: vec![Bool::True, Bool::False],
            },
        ));
        let actual = list("[true,false]");
        assert_eq!(expected, actual);
    }

    #[test]
    fn empty_with_padding() {
        let expected = Ok(("", List { values: vec![] }));
        let actual = list("[ ]");
        assert_eq!(expected, actual);

        let actual = list("[\n]");
        assert_eq!(expected, actual);

        let actual = list("[\t]");
        assert_eq!(expected, actual);

        let actual = list("[\n\n]");
        assert_eq!(expected, actual);

        let actual = list("[\t\t]");
        assert_eq!(expected, actual);

        let actual = list("[ \n\t]");
        assert_eq!(expected, actual);
    }

    #[test]
    fn single_with_padding() {
        let expected = Ok((
            "",
            List {
                values: vec![Bool::True],
            },
        ));
        let actual = list("[ true ]");
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_with_padding() {
        let expected = Ok((
            "",
            List {
                values: vec![Bool::True, Bool::False],
            },
        ));
        let actual = list("[ true,false ]");
        assert_eq!(expected, actual);

        let actual = list("[ true ,false ]");
        assert_eq!(expected, actual);

        let actual = list("[ true, false ]");
        assert_eq!(expected, actual);

        let actual = list("[ true , false ]");
        assert_eq!(expected, actual);

        let actual = list("[ true\t,\tfalse ]");
        assert_eq!(expected, actual);

        let actual = list("[ true\n,\nfalse ]");
        assert_eq!(expected, actual);
    }
}
