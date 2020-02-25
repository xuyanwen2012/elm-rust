pub mod typechecker;

#[cfg(test)]
mod tests {
    use rustelm_parser::parser::parse;

    #[test]
    fn it_works() {
        parse("11")
    }
}
