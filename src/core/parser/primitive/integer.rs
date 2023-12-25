use nom::character::complete::i32;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Integer(pub i32);

pub fn integer(input: &str) -> IResult<&str, Integer> {
    i32(input).map(|(remaining, value)| (remaining, Integer(value)))
}

#[cfg(test)]
mod tests {
    use crate::core::parser::primitive::integer::{integer, Integer};

    #[test]
    fn test_zero() {
        let expected = Ok(("", Integer(0)));
        let actual = integer("0");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_positive() {
        let expected = Ok(("", Integer(1)));
        let actual = integer("1");
        assert_eq!(expected, actual);

        let expected = Ok(("", Integer(1234)));
        let actual = integer("1234");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_negative() {
        let expected = Ok(("", Integer(-1)));
        let actual = integer("-1");
        assert_eq!(expected, actual);

        let expected = Ok(("", Integer(-1234)));
        let actual = integer("-1234");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_positive_max() {
        let max = i32::MAX.to_string();
        let expected = Ok(("", Integer(i32::MAX)));
        let actual = integer(max.as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_negative_min() {
        let min = i32::MIN.to_string();
        let expected = Ok(("", Integer(i32::MIN)));
        let actual = integer(min.as_str());
        assert_eq!(expected, actual);
    }
}
