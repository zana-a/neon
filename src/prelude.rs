pub mod parser {
    use nom::character::complete::anychar;
    use nom::character::complete::char;
    use nom::combinator::verify;
    use nom::error::Error;
    use nom::IResult;

    pub type Result<T, U> = IResult<T, U, Error<T>>;

    pub fn alpha(s: &str) -> self::Result<&str, char> {
        verify(anychar, |&c| c.is_alphabetic())(s)
    }

    pub fn numeric(s: &str) -> self::Result<&str, char> {
        verify(anychar, |&c| c.is_numeric())(s)
    }

    pub fn underscore(s: &str) -> self::Result<&str, char> {
        char('_')(s)
    }
}
