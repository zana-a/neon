pub type Result<I, O, E = (I, nom::error::ErrorKind)> =
  std::result::Result<(I, O), nom::Err<E>>;
