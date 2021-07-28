use super::ops::BinOp;
use crate::mion::expressions::Expression::Identifier;

struct Block {
    statements: Vec<Statement>,
}

struct Statement {
    expression: Box<Expression>
}

pub(crate) enum Expression {
    Identifier(String),
    Literal(Literal),
    Binary(Box<Expression>, BinOp, Box<Expression>)
}

impl Expression {
    pub(crate) fn new_identifier(id_str: &str) -> Expression {
        Identifier(id_str.to_string())
    }
}

pub(crate) enum Literal {
    Int(i64),
    String(String),
    Float(f64)
}