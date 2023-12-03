use nom::IResult;

pub fn left_brace(input: &str) -> IResult<&str, char> {
    nom::character::complete::char('{')(input)
}

pub fn right_brace(input: &str) -> IResult<&str, char> {
    nom::character::complete::char('}')(input)
}
