use crate::Types;
use im::HashMap;
use rustelm_parser::ast::{Atom, Expr};

type Context = im::HashMap<String, Types>;

pub fn typecheck_root(root: Box<Expr>) -> Types {
    let env = Context::new();

    typecheck(&env, root)
}

fn get_type_from_ctx(env: &Context, name: String) -> Types {
    // Could not find this var in env, need to Rise error
    env.get(name.as_str()).unwrap().clone()
}

fn typecheck(env: &Context, term: Box<Expr>) -> Types {
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
            // TODO: use rust idioms
            let mut new_env = env.clone();

            for (name, e) in bindings {
                new_env.insert(name, typecheck(env, e));
            }

            Unit
        }
        Expr::Signal(_) => Unit,
        Expr::Lift(_, _) => Unit,
        Expr::Foldp(_, _, _) => Unit,
    }
}

mod test {
    use super::Types;
    use crate::typechecker::{typecheck, typecheck_root, Context};
    use im::HashMap;
    use rustelm_parser::parser::parse;

    #[test]
    fn test_hashmap() {
        let env = hashmap! { "x".to_owned() => Types::Unit };
        assert_eq!(&format!("{:?}", env), "{\"x\": Unit}");

        let mut new_env = env.clone();
        new_env.insert(String::from("y"), Types::Unit);

        assert!(new_env.contains_key("x"));
        assert!(new_env.contains_key("y"));
    }

    #[test]
    fn test_atom() {
        assert_eq!(typecheck_root(parse("1").unwrap()), Types::Int);
        assert_eq!(typecheck_root(parse("()").unwrap()), Types::Unit);

        let fake_env = hashmap! { "x".to_owned() => Types::Int };

        assert_eq!(typecheck(&fake_env, parse("x").unwrap()), Types::Int);
    }
}
