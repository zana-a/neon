use nom::{character::complete::char, IResult};

pub fn left_brace(input: &str) -> IResult<&str, char> {
    char('{')(input)
}

pub fn right_brace(input: &str) -> IResult<&str, char> {
    char('}')(input)
}
