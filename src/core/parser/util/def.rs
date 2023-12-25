use nom::bytes::complete::tag;
use nom::IResult;

pub fn def(input: &str) -> IResult<&str, &str> {
    tag("def")(input)
}
