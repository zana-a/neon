use nom::{
    character::complete::multispace0,
    sequence::{preceded, terminated},
    IResult,
};

use super::{
    boolean::{boolean, Boolean},
    semi_colon::semi_colon,
};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Boolean(Boolean),
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    terminated(boolean, preceded(multispace0, semi_colon))(input)
        .map(|(remaining, result)| (remaining, Expression::Boolean(result)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn boolean_expression_zero_space() {
        let expected = Ok(("", Expression::Boolean(Boolean::True)));
        let result = expression("true;");
        assert_eq!(expected, result);

        let expected = Ok(("", Expression::Boolean(Boolean::False)));
        let result = expression("false;");
        assert_eq!(expected, result);
    }

    #[test]
    fn boolean_expression_many_space() {
        let expected = Ok(("", Expression::Boolean(Boolean::True)));
        let result = expression("true   ;");
        assert_eq!(expected, result);

        let expected = Ok(("", Expression::Boolean(Boolean::True)));
        let result = expression("true\n\n\n;");
        assert_eq!(expected, result);

        let expected = Ok(("", Expression::Boolean(Boolean::True)));
        let result = expression("true\t\t\t;");
        assert_eq!(expected, result);
    }
}
