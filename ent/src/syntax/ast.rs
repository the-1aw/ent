use crate::syntax::{token_type::Literal, Token};
use std::fmt::Display;

pub enum Expr {
    Binary {
        l: Box<Expr>,
        op: Token,
        r: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Unary {
        op: Token,
        r: Box<Expr>,
    },
    // Terminals
    Literal(Literal),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary { op, l, r } => write!(f, "({op} {l} {r})"),
            Expr::Grouping(expr) => write!(f, "(group {expr})"),
            Expr::Literal(lit) => write!(f, "{lit}"),
            Expr::Unary { op, r } => write!(f, "({op} {r})"),
        }
    }
}
