use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Const(Atom), // Unit, Num, and Variables
    Abs(Vec<String>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let((String, Box<Expr>), Box<Expr>),
    Signal(String), // Input
    Lift,
    Foldp,
}

pub enum Atom {
    Unit,
    Num(i32),
    Var(String),
}

#[derive(Copy, Clone)]
pub enum BinOp {
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
                Atom::Unit => write!(fmt, "()"),
                Atom::Num(num) => write!(fmt, "{:?}", num),
                Atom::Var(str) => write!(fmt, "{:?}", str),
            },
            Abs(ref vec, ref e1) => {
                write!(fmt, "(\\").unwrap();
                for id in vec {
                    write!(fmt, " {:?}", id).unwrap();
                }
                write!(fmt, " -> {:?})", e1)
            }
            App(ref e1, ref e2) => write!(fmt, "({:?} {:?})", e1, e2),
            BinOp(ref e1, op, ref e2) => write!(fmt, "({:?} {:?} {:?})", e1, op, e2),
            If(ref pred, ref e1, ref e2) => write!(
                fmt,
                "if ( {:?} ) then {{ {:?} }} else {{ {:?} }}",
                pred, e1, e2
            ),
            Let((ref binder, ref value), ref e1) => {
                write!(fmt, "let {:?} = {:?} in {:?}", binder, value, e1)
            }
            Signal(_) => write!(fmt, ""),
            Lift => write!(fmt, ""),
            Foldp => write!(fmt, ""),
        }
    }
}

impl Debug for BinOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::BinOp::*;
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
