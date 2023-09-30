use nom::{character::complete::anychar, combinator::verify, IResult};

pub fn alpha(input: &str) -> IResult<&str, char> {
    verify(anychar, |&c| c.is_alphabetic())(input)
}
