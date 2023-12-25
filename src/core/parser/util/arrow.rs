use nom::bytes::complete::tag;
use nom::IResult;

pub fn right_arrow(input: &str) -> IResult<&str, &str> {
    tag("->")(input)
}
