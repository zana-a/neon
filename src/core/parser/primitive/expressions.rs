use nom::character::complete::multispace1;
use nom::multi::separated_list0;
use nom::IResult;

use crate::core::parser::construct::expression::{expression, Expression};

#[derive(Debug, PartialEq)]
pub struct Expressions {
    pub expressions: Vec<Expression>,
}

pub fn expressions(input: &str) -> IResult<&str, Vec<Expression>> {
    separated_list0(multispace1, expression)(input)
}
