use nom::{character::complete::char, IResult};

pub fn comma(input: &str) -> IResult<&str, char> {
    char(',')(input)
}
