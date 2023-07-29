use nom::error::Error;
use nom::IResult;

pub type Result<T, U> = IResult<T, U, Error<T>>;
