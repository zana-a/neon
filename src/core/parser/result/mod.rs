/// Removes the need for using the weird Nom result type.
/// This could potentially be updated to hold more custom results.
pub type Result<I, O, E = (I, nom::error::ErrorKind)> = std::result::Result<(I, O), nom::Err<E>>;
