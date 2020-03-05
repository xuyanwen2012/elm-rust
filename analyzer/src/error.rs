use std::{
    error, fmt,
    fmt::{Error, Formatter},
};

pub type Result<T> = std::result::Result<T, TypeCheckError>;

#[derive(Debug, Clone)]
pub struct TypeCheckError(pub TypeCheckErrorType);

#[derive(Debug, Clone)]
pub enum TypeCheckErrorType {
    TypeMissMatch,
    UndefinedName,
}

impl fmt::Display for TypeCheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use TypeCheckErrorType::*;
        match self.0 {
            TypeMissMatch => write!(f, "type miss matched"),
            UndefinedName => write!(f, "undefined variable"),
        }
    }
}

impl error::Error for TypeCheckError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
