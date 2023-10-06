use nom::{character::complete::char, IResult};

pub fn colon(input: &str) -> IResult<&str, char> {
    char(':')(input)
}
