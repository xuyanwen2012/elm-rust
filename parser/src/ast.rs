use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Const(Constant),
    Abs(Vec<String>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

pub enum Constant {
    Unit,
    Num(i32),
    Ident(String),
}

// pub enum Expr {
//     Unit,
//     Number(i32),
//     Lambda(Vec<String>, Box<Expr>),
//     App(Box<Expr>, Box<Expr>),
//     Identifier(String),
//     BinOp(Box<Expr>, Opcode, Box<Expr>),
//     If(Box<Expr>, Box<Expr>, Box<Expr>),
//     Let(Vec<(String, Box<Expr>)>, Box<Expr>),
//     Error,
// }

#[derive(Copy, Clone)]
pub enum Opcode {
    // Arithmetic
    Mul,
    Div,
    Add,
    Sub,
    // Comparison
    Eq,
    Ne,
    Le,
    Ge,
    Less,
    Greater,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Const(ref c) => match c {
                Constant::Unit => write!(fmt, "()"),
                Constant::Num(num) => write!(fmt, "{:?}", num),
                Constant::Ident(str) => write!(fmt, "{:?}", str),
            },
            Abs(ref vec, ref e1) => {
                write!(fmt, "(\\");
                for id in vec {
                    write!(fmt, " {:?}", id);
                }
                write!(fmt, " -> {:?})", e1)
            }
            App(ref e1, ref e2) => write!(fmt, "({:?} {:?})", e1, e2),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Eq => write!(fmt, "=="),
            Ne => write!(fmt, "!="),
            Le => write!(fmt, "<="),
            Ge => write!(fmt, ">="),
            Less => write!(fmt, "<"),
            Greater => write!(fmt, ">"),
        }
    }
}
