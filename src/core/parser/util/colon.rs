use nom::IResult;

pub fn colon(input: &str) -> IResult<&str, char> {
    nom::character::complete::char(':')(input)
}
