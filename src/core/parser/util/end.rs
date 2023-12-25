use nom::bytes::complete::tag;
use nom::IResult;

pub fn end(input: &str) -> IResult<&str, &str> {
    tag("end")(input)
}
