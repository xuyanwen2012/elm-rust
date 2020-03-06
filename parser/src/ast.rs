use num_bigint::BigInt;
use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Const(Atom), // Unit, Num, and Variables
    Abs(Atom, Types, Box<Expr>),
    // Abs(Vec<String>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Atom, Box<Expr>, Box<Expr>),
    Signal(String), // Input
    Lift(usize, Vec<Expr>),
    Foldp(Box<Expr>, Box<Expr>, Box<Expr>),
}

pub enum Atom {
    Unit,
    Num(BigInt),
    Var(String),
}

/// Simple types
/// t ::= unit | int | t -> t'
pub enum Types {
    Unit,
    Int,
    Abs(Box<Types>, Box<Types>),
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
            Const(ref c) => write!(fmt, "{:?}", c),
            Abs(ref atom, ref ty, ref e1) => match atom {
                Atom::Var(ref name) => write!(fmt, "\\{:?}: {:?}. -> {:?}", name, ty, e1),
                _ => unreachable!(),
            },
            App(ref e1, ref e2) => write!(fmt, "({:?} {:?})", e1, e2),
            BinOp(ref e1, op, ref e2) => write!(fmt, "({:?} {:?} {:?})", e1, op, e2),
            If(ref pred, ref e1, ref e2) => write!(
                fmt,
                "if ( {:?} ) then {{ {:?} }} else {{ {:?} }}",
                pred, e1, e2
            ),
            Let(ref atom, ref e1, ref e2) => match atom {
                Atom::Var(ref name) => write!(fmt, "let {:?} = {:?} in {:?}", name, e1, e2),
                _ => unreachable!(),
            },
            Signal(_) => write!(fmt, ""),
            Lift(_, _) => write!(fmt, ""),
            Foldp(_, _, _) => write!(fmt, ""),
        }
    }
}

impl Debug for Atom {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Atom::*;
        match *self {
            Unit => write!(fmt, "()"),
            Num(ref big_int) => write!(
                fmt,
                "{:?}",
                big_int.to_str_radix(10).parse::<i32>().unwrap()
            ),
            Var(ref str) => write!(fmt, "{:?}", str),
        }
    }
}

impl Debug for Types {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Types::*;
        match *self {
            Unit => write!(fmt, "unit"),
            Int => write!(fmt, "int"),
            Abs(ref t1, ref t2) => write!(fmt, "({:?} -> {:?})", t1, t2),
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
