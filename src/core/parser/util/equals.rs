use nom::IResult;

pub fn equals(input: &str) -> IResult<&str, char> {
    nom::character::complete::char('=')(input)
}
