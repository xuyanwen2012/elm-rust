extern crate lalrpop_util;

use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod tokens;

lalrpop_mod!(
    #[allow(clippy::all)]
    pub elm
);
