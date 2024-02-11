use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser;

pub fn padded0<'a, F, O, E>(
  inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
  F: Parser<&'a str, O, E>,
  E: ParseError<&'a str>,
{
  delimited(multispace0, inner, multispace0)
}

pub fn padded1<'a, F, O, E>(
  inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
  F: Parser<&'a str, O, E>,
  E: ParseError<&'a str>,
{
  delimited(multispace1, inner, multispace1)
}
