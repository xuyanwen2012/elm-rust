#[macro_use]
extern crate im;
#[macro_use]
extern crate lazy_static;

mod error;
pub mod typechecker;

/// The simple types in the language.
#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Unit,
    Int,
    Abs(Box<Types>, Box<Types>),
}

#[cfg(test)]
mod tests {
    use rustelm_parser::parser::parse;

    #[test]
    fn it_works() {
        assert!(parse("11").is_ok())
    }
}
