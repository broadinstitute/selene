use crate::util::error::Error;
use super::values::Value;
use crate::mion::syntax::ops::BinOp;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::symbols::{Symbols, VarEntry};
use crate::mion::eval::predef;
use std::collections::HashMap;

pub(crate) enum Expression {
    Identifier(Identifier),
    Value(Value),
    Binary(Box<Expression>, BinOp, Box<Expression>),
    Member(Box<Expression>, Identifier),
    Call(Box<Expression>, Vec<Assignment>),
    Scatter(Box<Scatter>),
    Assignment(Box<Assignment>),
    Block(Box<Block>),
}

impl Expression {
    pub(crate) fn evaluate(&self, symbols: &Symbols) -> Result<Value, Error> {
        match self {
            Expression::Identifier(identifier) => {
                match symbols.var_entries.get(identifier) {
                    None => {
                        Err(Error::from(format!("Unknown variable {}", identifier)))
                    }
                    Some(var_entry) => {
                        match var_entry {
                            VarEntry::Uninitialized => {
                                Err(Error::from(format!("Uninitialized variable {}",
                                                        identifier)))
                            }
                            VarEntry::Value(value) => { Ok(value.clone()) }
                        }
                    }
                }
            }
            Expression::Value(value) => { Ok(value.clone()) }
            Expression::Binary(_, _, _) => { todo!() }
            Expression::Member(_, _) => { todo!() }
            Expression::Call(callee, args) => {
                let callee_value = callee.evaluate(symbols)?;
                if let Value::Function(function) = callee_value {
                    let mut args_map = HashMap::<Identifier, Value>::new();
                    for arg in args {
                        let identifier = &arg.lhs;
                        let value = arg.rhs.evaluate(symbols)?;
                        args_map.insert(identifier.clone(), value);
                    }
                    function.call(args_map)
                } else {
                    Err(Error::from(format!("Expected function, but got {}",
                                            callee_value)))
                }
            }
            Expression::Scatter(_) => { todo!() }
            Expression::Assignment(assignment) => {
                assignment.rhs.evaluate(symbols)
            }
            Expression::Block(_) => { todo!() }
        }
    }
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

pub(crate) trait Function {
    fn id(&self) -> &str;
    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error>;
}

pub(crate) struct Script {
    expressions: Vec<Expression>,
}

impl Script {
    pub(crate) fn new(expressions: Vec<Expression>) -> Script { Script { expressions } }
    pub(crate) fn optimize(self) -> Script { self }
    pub(crate) fn evaluate(&self) -> Result<Value, Error> {
        let symbols = predef::predef_symbols();
        evaluate_expressions(&self.expressions, &symbols)
    }
}

fn evaluate_expressions(expressions: &[Expression], symbols: &Symbols) -> Result<Value, Error> {
    let mut symbols_local = symbols.clone();
    let mut value = Value::Unit;
    for expression in expressions {
        value = expression.evaluate(&symbols_local)?;
        if let Expression::Assignment(assignment) = expression {
            let lhs = &assignment.lhs;
            symbols_local = symbols_local.with_var_value_entry(&lhs, &value)
        }
    }
    Ok(value)
}

