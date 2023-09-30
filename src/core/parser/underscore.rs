use nom::{character::complete::char, IResult};

pub fn underscore(input: &str) -> IResult<&str, char> {
    char('_')(input)
}
