pub mod ast;
pub mod parser;

extern crate lalrpop_util;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(dead_code)]
    pub elm
);
