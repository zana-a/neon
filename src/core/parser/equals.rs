use nom::{character::complete::char, IResult};

pub fn equals(input: &str) -> IResult<&str, char> {
    char('=')(input)
}
