#[macro_use]
extern crate lazy_static;

pub mod typechecker;

#[cfg(test)]
mod tests {
    use rustelm_parser::parser::parse;

    #[test]
    fn it_works() {
        assert!(parse("11").is_ok())
    }
}
