use nom::{character::complete::char, IResult};

pub fn semi_colon(input: &str) -> IResult<&str, char> {
    char(';')(input)
}
