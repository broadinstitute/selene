use crate::mion::syntax::ops::BinOp;
use std::fmt::{Display, Formatter};
use crate::Error;
use crate::mion::eval::eval;
use crate::util::iter_util::fmt_vec;
use crate::mion::eval::symbols::{Symbols, SymbolsExpressionResult, VarEntry};
use crate::mion::eval::values::Value;

pub(crate) struct Script {
    pub(crate) expressions: Vec<Expression>,
}

impl Script {
    pub(crate) fn new(expressions: Vec<Expression>) -> Script {
        Script { expressions }
    }
    pub(crate) fn compile(&self) -> Result<eval::Script, Error> {
        let mut eval_expressions = Vec::<eval::Expression>::new();
        todo!();
        Ok(eval::Script::new(eval_expressions))
    }
}

pub(crate) struct Block {
    pub(crate) expressions: Vec<Expression>,
}

pub(crate) struct Assignment {
    pub(crate) lhs: Identifier,
    pub(crate) rhs: Expression,
}

pub(crate) struct Scatter {
    pub(crate) iteration: Iteration,
    pub(crate) expression: Box<Expression>,
}

pub(crate) struct Iteration {
    pub(crate) lhs: Identifier,
    pub(crate) rhs: Expression,
}

pub(crate) enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Binary(Box<Expression>, BinOp, Box<Expression>),
    Member(Box<Expression>, Identifier),
    Call(Box<Expression>, Vec<Expression>),
    Scatter(Box<Scatter>),
    Assignment(Box<Assignment>),
    Block(Block),
}

impl Expression {
    pub(crate) fn compile(&self, symbols: &Symbols) -> Result<eval::Expression, Error> {
        match self {
            Expression::Identifier(identifier) => {
                match symbols.var_entries.get(identifier) {
                    None => {
                        Err(Error::from(format!("Unknown variable {}.", identifier)))
                    }
                    Some(VarEntry::Uninitialized) => {
                        Ok(eval::Expression::Identifier(identifier.name.clone()))
                    }
                    Some(VarEntry::Value(value)) => {
                        Ok(eval::Expression::Value(*value.clone()))
                    }
                }
            }
            Expression::Literal(literal) => {
                Ok(eval::Expression::Value(literal.to_value()))
            }
            Expression::Binary(lhs, op, rhs) => {
                let eval_lhs = Box::new(lhs.compile(symbols)?);
                let eval_rhs = Box::new(rhs.compile(symbols)?);
                Ok(eval::Expression::Binary(eval_lhs, *op,
                                            eval_rhs))
            }
            Expression::Member(expression, identifier) => {

            }
            Expression::Call(_, _) => {}
            Expression::Scatter(_) => {}
            Expression::Assignment(_) => {}
            Expression::Block(_) => {}
        }
    }
}

#[derive(PartialEq)]
pub(crate) struct Identifier {
    pub(crate) name: String,
}

impl Identifier {
    pub(crate) fn new(name: String) -> Identifier { Identifier { name } }
}

pub(crate) enum Literal {
    Int(i64),
    String(String),
    Float(f64),
}

impl Literal {
    pub(crate) fn to_value(&self) -> Value {
        match self {
            Literal::Int(int) => { Value::Int(*int) }
            Literal::String(string) => { Value::String(string.clone()) }
            Literal::Float(float) => { Value::Float(*float) }
        }
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for expression in &self.expressions {
            format!("{};\n", expression).fmt(f)?;
        }
        Ok(())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(identifier) => { identifier.fmt(f) }
            Expression::Literal(literal) => { literal.fmt(f) }
            Expression::Binary(lhs, op, rhs) => {
                format!("{} {} {}", lhs, op.symbol(), rhs).fmt(f)
            }
            Expression::Member(expression, identifier) => {
                format!("{}.{}", expression, identifier).fmt(f)
            }
            Expression::Call(callee, args) => {
                callee.fmt(f)?;
                fmt_vec("(", args, ")", f)
            }
            Expression::Scatter(scatter) => { scatter.fmt(f) }
            Expression::Assignment(assignment) => { assignment.fmt(f) }
            Expression::Block(block) => { block.fmt(f) }
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(int) => { int.fmt(f) }
            Literal::String(string) => { string.fmt(f) }
            Literal::Float(float) => { float.fmt(f) }
        }
    }
}

impl Display for Scatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("({}) {}", self.iteration, self.expression).fmt(f)
    }
}

impl Display for Iteration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("{} <- {}", self.lhs, self.rhs).fmt(f)
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("{} = {}", self.lhs, self.rhs).fmt(f)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        "{\n".fmt(f)?;
        for expression in &self.expressions {
            format!("  {};\n", expression).fmt(f)?;
        }
        "}".fmt(f)
    }
}