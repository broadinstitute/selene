use super::ops::BinOp;

struct Block {
    statements: Vec<Statement>,
}

struct Statement {
    expression: Box<Expression>
}

pub(crate) enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Binary(Box<Expression>, BinOp, Box<Expression>),
    Member(Box<Expression>, Identifier),
    Call(Box<Expression>, Vec<Expression>)
}

impl Expression {
    pub(crate) fn new_identifier(id_str: &str) -> Expression {
        Expression::Identifier(Identifier::new(id_str.to_string()))
    }
}

pub(crate) struct Identifier {
    name: String
}

impl Identifier {
    pub(crate) fn new(name: String) -> Identifier { Identifier { name} }
}

pub(crate) enum Literal {
    Int(i64),
    String(String),
    Float(f64)
}