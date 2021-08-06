use crate::util::error::Error;
use super::symbols::Symbols;
use super::values::Value;
use crate::mion::syntax::ops::BinOp;

pub(crate) enum Expression {
    Identifier(String),
    Value(Value),
    Binary(Box<Expression>, BinOp, Box<Expression>)
}


pub(crate) struct Script {
    expressions: Vec<Expression>
}

impl Script {
    pub(crate) fn new(expressions: Vec<Expression>) -> Script { Script { expressions } }
    pub(crate) fn optimize(self) -> Script { self }
    pub(crate) fn evaluate(&self) -> Result<Value, Error> { unimplemented!() }
}

