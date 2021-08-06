use crate::util::error::Error;
use super::values::Value;
use crate::mion::syntax::ops::BinOp;
use crate::mion::eval::identifier::Identifier;

pub(crate) enum Expression {
    Identifier(Identifier),
    Value(Value),
    Binary(Box<Expression>, BinOp, Box<Expression>),
    Member(Box<Expression>, Identifier),
    Call(Box<Expression>, Vec<Expression>),
    Scatter(Box<Scatter>),
    Assignment(Box<Assignment>),
    Block(Box<Block>),
}

pub(crate) struct Scatter {
    pub(crate) iteration: Iteration,
    pub(crate) expression: Expression,
}

impl Scatter {
    pub(crate) fn new(iteration: Iteration, expression: Expression) -> Scatter {
        Scatter { iteration, expression }
    }
}

pub(crate) struct Iteration {
    pub(crate) lhs: Identifier,
    pub(crate) rhs: Expression,
}

impl Iteration {
    pub(crate) fn new(lhs: Identifier, rhs: Expression) -> Iteration {
        Iteration { lhs, rhs }
    }
}

pub(crate) struct Assignment {
    pub(crate) lhs: Identifier,
    pub(crate) rhs: Expression,
}

impl Assignment {
    pub(crate) fn new(lhs: Identifier, rhs: Expression) -> Assignment {
        Assignment { lhs, rhs }
    }
}

pub(crate) struct Block {
    expressions: Vec<Expression>,
}

impl Block {
    pub(crate) fn new(expressions: Vec<Expression>) -> Block {
        Block { expressions }
    }
}

pub(crate) struct Script {
    expressions: Vec<Expression>,
}

impl Script {
    pub(crate) fn new(expressions: Vec<Expression>) -> Script { Script { expressions } }
    pub(crate) fn optimize(self) -> Script { self }
    pub(crate) fn evaluate(&self) -> Result<Value, Error> { unimplemented!() }
}

