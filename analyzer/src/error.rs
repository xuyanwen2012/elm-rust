use std::{
    error, fmt,
    fmt::{Error, Formatter},
};

pub type Result<T> = std::result::Result<T, TypeMismatchError>;

#[derive(Debug, Clone)]
pub struct TypeMismatchError;

impl fmt::Display for TypeMismatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "type miss matched")
    }
}

impl error::Error for TypeMismatchError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
