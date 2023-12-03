//! # Primitive Module
//! This module defines parsers that are not usually useful on their own and therefore
//! commonly require composition with other parsers.

pub(super) mod boolean;
pub(super) mod identifier;
mod integer;
pub(super) mod pair;
