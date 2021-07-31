use crate::mion::syntax::ops::BinOp;

pub(crate) struct Block {
    pub(crate) expressions: Vec<Expression>,
}

pub(crate) struct Assignment {
    pub(crate) lhs: Identifier,
    pub(crate) rhs: Expression
}

pub(crate) struct Scatter {
    pub(crate) iteration: Iteration,
    pub(crate) expression: Box<Expression>
}

pub(crate) struct Iteration {
    pub(crate) lhs: Identifier,
    pub(crate) rhs: Expression
}

pub(crate) enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Binary(Box<Expression>, BinOp, Box<Expression>),
    Member(Box<Expression>, Identifier),
    Call(Box<Expression>, Vec<Expression>),
    Scatter(Box<Scatter>),
    Assignment(Box<Assignment>),
    Block(Block)
}

impl Expression {
    pub(crate) fn new_identifier(id_str: &str) -> Expression {
        Expression::Identifier(Identifier::new(id_str.to_string()))
    }
}

pub(crate) struct Identifier {
    pub(crate) name: String
}

impl Identifier {
    pub(crate) fn new(name: String) -> Identifier { Identifier { name} }
}

pub(crate) enum Literal {
    Int(i64),
    String(String),
    Float(f64)
}