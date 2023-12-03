//! # Expression module
//!
//! An expression is a construct that can produce an output. It can be plugged into many places
//! within the language.
//!
//! Reference:
//! ```
//! <expression>;
//! ```
//!
//! Example:
//! ```
//! 23;
//! true;
//! [1, 2, 3, 4];
//! ```

use nom::IResult;

// todo
pub struct Expression {}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    todo!()
}
