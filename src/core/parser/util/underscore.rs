use nom::IResult;

pub fn underscore(input: &str) -> IResult<&str, char> {
    nom::character::complete::char('_')(input)
}
