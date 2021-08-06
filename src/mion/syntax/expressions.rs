use crate::mion::syntax::ops::BinOp;
use std::fmt::{Display, Formatter};
use crate::Error;
use crate::mion::eval::eval;
use crate::util::iter_util::fmt_vec;
use crate::mion::eval::symbols::{Symbols, VarEntry};
use crate::mion::eval::values::Value;
use crate::mion::eval::identifier::Identifier;

pub(crate) struct Script {
    pub(crate) expressions: Vec<Expression>,
}

impl Script {
    pub(crate) fn new(expressions: Vec<Expression>) -> Script {
        Script { expressions }
    }
    pub(crate) fn compile(&self) -> Result<eval::Script, Error> {
        let mut eval_expressions = Vec::<eval::Expression>::new();
        let mut symbols = Symbols::new();
        for expression in &self.expressions {
            let eval_expression = expression.compile(&symbols)?;
            if let eval::Expression::Assignment(assignment) = &eval_expression {
                let identifier = &assignment.lhs;
                let rhs = &assignment.rhs;
                if let eval::Expression::Value(value) = rhs {
                    symbols = symbols.with_var_value_entry(&identifier, &value);
                } else {
                    symbols = symbols.with_var_uninitialized_entry(&identifier);
                }
            }
            eval_expressions.push(eval_expression);
        }
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
                        Ok(eval::Expression::Identifier(identifier.clone()))
                    }
                    Some(VarEntry::Value(value)) => {
                        Ok(eval::Expression::Value(value.clone()))
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
                let eval_expression = Box::new(expression.compile(symbols)?);
                Ok(eval::Expression::Member(eval_expression,
                                            identifier.clone()))
            }
            Expression::Call(callee, arguments) => {
                let eval_callee = Box::new(callee.compile(symbols)?);
                let mut eval_arguments = Vec::<eval::Expression>::new();
                for argument in arguments {
                    eval_arguments.push(argument.compile(symbols)?);
                }
                Ok(eval::Expression::Call(eval_callee, eval_arguments))
            }
            Expression::Scatter(scatter) => {
                let eval_iteration_lhs = scatter.iteration.lhs.clone();
                let eval_iteration_expression = scatter.iteration.rhs.compile(symbols)?;
                let eval_expression = scatter.expression.compile(symbols)?;
                let eval_iteration =
                    eval::Iteration::new(eval_iteration_lhs, eval_iteration_expression);
                let eval_scatter = eval::Scatter::new(eval_iteration, eval_expression);
                Ok(eval::Expression::Scatter(Box::new(eval_scatter)))
            }
            Expression::Assignment(assignment) => {
                let eval_identifier = assignment.lhs.clone();
                let eval_expression = assignment.rhs.compile(symbols)?;
                let eval_assignment =
                    eval::Assignment::new(eval_identifier, eval_expression);
                Ok(eval::Expression::Assignment(Box::new(eval_assignment)))
            }
            Expression::Block(block) => {
                let mut eval_expressions = Vec::new();
                for expression in &block.expressions {
                    eval_expressions.push(expression.compile(symbols)?);
                }
                let eval_block = eval::Block::new(eval_expressions);
                Ok(eval::Expression::Block(Box::new(eval_block)))
            }
        }
    }
}

pub(crate) enum Literal {
    Int(i64),
    String(String),
    Float(f64),
}

impl Literal {
    pub(crate) fn to_value(&self) -> Value {
        match self {
            Literal::Int(int) => { Value::from(int) }
            Literal::String(string) => { Value::from(string) }
            Literal::Float(float) => { Value::from(float) }
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