use nom::IResult;

pub fn comma(input: &str) -> IResult<&str, char> {
    nom::character::complete::char(',')(input)
}
