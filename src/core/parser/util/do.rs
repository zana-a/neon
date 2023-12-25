use nom::bytes::complete::tag;
use nom::IResult;

pub fn r#do(input: &str) -> IResult<&str, &str> {
    tag("do")(input)
}
