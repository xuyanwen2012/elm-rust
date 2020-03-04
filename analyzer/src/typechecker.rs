use std::collections::HashMap;

use rustelm_parser::ast;
use rustelm_parser::ast::{Atom, Expr};

type Context = HashMap<u32, String>;

#[derive(PartialEq)]
pub enum Types {
    Unit,
    Int,
    Abs(Box<Types>, Box<Types>),
}

pub fn typecheck_root(root: Box<ast::Expr>) -> Types {
    let env = Context::new();

    typecheck(&env, root)
}

fn get_type_from_ctx(env: &Context, name: String) -> Types {
    Types::Unit
}

fn typecheck(env: &Context, term: Box<ast::Expr>) -> Types {
    use Types::*;
    match *term {
        Expr::Const(atom) => match atom {
            Atom::Unit => Unit,
            Atom::Num(_) => Int,
            Atom::Var(var) => get_type_from_ctx(env, var),
        },
        Expr::Abs(args, expr) => {
            // Add bindings to context
            // TODO: Fix this

            for arg in args {}

            Unit
        }
        Expr::App(e1, e2) => {
            let type_e1 = typecheck(env, e1);
            let type_e2 = typecheck(env, e2);

            match type_e1 {
                Abs(t11, t12) => {
                    if type_e2 == *t11 {
                        *t12
                    } else {
                        // TODO: Error
                        Unit
                    }
                }
                // TODO: Error
                _ => Unit,
            }
        }
        Expr::BinOp(l, _, r) => {
            if Int == typecheck(env, r) && Int == typecheck(env, l) {
                Int
            } else {
                // TODO: Error
                Unit
            }
        }
        Expr::If(pred, e1, e2) => {
            if Int == typecheck(env, pred) {
                let type_e1 = typecheck(env, e1);
                if type_e1 == typecheck(env, e2) {
                    type_e1
                } else {
                    // TODO: Error
                    Unit
                }
            } else {
                // TODO: Error
                Unit
            }
        }
        Expr::Let(bindings, expr) => {
            // Add bindings

            Unit
        }
        Expr::Signal(_) => Unit,
        Expr::Lift(_, _) => Unit,
        Expr::Foldp(_, _, _) => Unit,
    }
}

mod test {
    #[test]
    fn test_hashmap() {}
}
