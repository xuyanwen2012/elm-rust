extern crate lalrpop_util;

use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod lexer;
pub mod parser;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(dead_code)]
    pub elm
);
