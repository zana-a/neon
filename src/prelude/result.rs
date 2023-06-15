use nom::IResult;

use nom::error::Error;

pub type ParseResult<T, U> = IResult<T, U, Error<T>>;
