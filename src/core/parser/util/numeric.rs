use nom::{character::complete::anychar, combinator::verify, IResult};

pub fn numeric(input: &str) -> IResult<&str, char> {
    verify(anychar, |&c| c.is_numeric())(input)
}
