use nom::IResult;

pub fn semi_colon(input: &str) -> IResult<&str, char> {
    nom::character::complete::char(';')(input)
}
