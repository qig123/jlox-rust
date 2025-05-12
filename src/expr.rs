use std::fmt;

use crate::token::{Object, Token};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Object),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl Expr {
    pub fn pretty_print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let indent_str = " ".repeat(indent);
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                writeln!(f, "{}Binary:", indent_str)?;
                writeln!(f, "{}  operator: {}", indent_str, operator.lexeme)?;
                writeln!(f, "{}  left:", indent_str)?;
                left.pretty_print(f, indent + 4)?;
                writeln!(f, "{}  right:", indent_str)?;
                right.pretty_print(f, indent + 4)
            }
            Expr::Unary { operator, right } => {
                writeln!(f, "{}Unary:", indent_str)?;
                writeln!(f, "{}  operator: {}", indent_str, operator.lexeme)?;
                writeln!(f, "{}  expr:", indent_str)?;
                right.pretty_print(f, indent + 4)
            }
            Expr::Grouping(expr) => {
                writeln!(f, "{}Grouping:", indent_str)?;
                expr.pretty_print(f, indent + 2)
            }
            Expr::Literal(obj) => match obj {
                Object::Number(n) => writeln!(f, "{}Literal(Number({}))", indent_str, n),
                Object::String(s) => writeln!(f, "{}Literal(String(\"{}\"))", indent_str, s),
                Object::Boolean(b) => writeln!(f, "{}Literal(Boolean({}))", indent_str, b),
                Object::NULL => writeln!(f, "{}Literal(NULL)", indent_str),
            },
        }
    }
}
