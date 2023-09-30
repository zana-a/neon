use nom::{branch::alt, bytes::complete::tag, IResult};

#[derive(PartialEq, Debug)]
pub enum Bool {
    True,
    False,
}

fn t(input: &str) -> IResult<&str, Bool> {
    tag("true")(input).map(|(remaining, _)| (remaining, Bool::True))
}

fn f(input: &str) -> IResult<&str, Bool> {
    tag("false")(input).map(|(remaining, _)| (remaining, Bool::False))
}

pub fn boolean(input: &str) -> IResult<&str, Bool> {
    alt((t, f))(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t() {
        let expected = Ok(("", Bool::True));
        let actual = boolean("true");
        assert_eq!(expected, actual);
    }

    #[test]
    fn f() {
        let expected = Ok(("", Bool::False));
        let actual = boolean("false");
        assert_eq!(expected, actual);
    }
}
