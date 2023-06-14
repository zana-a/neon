use nom::IResult;
use std::fmt::Error;

pub type ParseResult<T, U> = IResult<T, U, Error>;
