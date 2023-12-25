use nom::IResult;

pub fn left_parenthesis(input: &str) -> IResult<&str, char> {
    nom::character::complete::char('(')(input)
}

pub fn right_parenthesis(input: &str) -> IResult<&str, char> {
    nom::character::complete::char(')')(input)
}
