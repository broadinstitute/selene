use crate::mion::syntax::ops::BinOp;
use std::fmt::{Display, Formatter};
use crate::Error;
use crate::mion::eval::{expressions, predef};
use crate::util::iter_util::fmt_vec;
use crate::mion::eval::symbols::{Symbols, VarEntry};
use crate::mion::eval::values::Value;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval;

pub(crate) struct Script {
    pub(crate) expressions: Vec<Expression>,
}

impl Script {
    pub(crate) fn new(expressions: Vec<Expression>) -> Script {
        Script { expressions }
    }
    pub(crate) fn compile(&self) -> Result<expressions::Script, Error> {
        let symbols = predef::predef_symbols();
        let eval_expressions = compile_expressions(&self.expressions, &symbols)?;
        Ok(expressions::Script::new(eval_expressions))
    }
}

fn compile_expressions(expressions: &[Expression], symbols: &Symbols)
                       -> Result<Vec<expressions::Expression>, Error> {
    let mut eval_expressions = Vec::<expressions::Expression>::new();
    let mut symbols_local = symbols.clone();
    for expression in expressions {
        let eval_expression = expression.compile(&symbols_local)?;
        if let expressions::Expression::Assignment(assignment) = &eval_expression {
            let identifier = &assignment.lhs;
            let rhs = &assignment.rhs;
            if let expressions::Expression::Value(value) = rhs {
                symbols_local = symbols_local.with_var_value_entry(identifier, value);
            } else {
                symbols_local = symbols_local.with_var_uninitialized_entry(identifier);
            }
        }
        eval_expressions.push(eval_expression);
    }
    Ok(eval_expressions)
}

pub(crate) struct Block {
    pub(crate) expressions: Vec<Expression>,
}

pub(crate) struct Assignment {
    pub(crate) lhs: Identifier,
    pub(crate) rhs: Expression,
}

impl Assignment {
    pub(crate) fn compile(&self, symbols: &Symbols)
                          -> Result<eval::expressions::Assignment, Error> {
        let eval_identifier = self.lhs.clone();
        let eval_expression = self.rhs.compile(symbols)?;
        Ok(eval::expressions::Assignment::new(eval_identifier, eval_expression))
    }
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
    Call(Box<Expression>, Vec<Assignment>),
    Scatter(Box<Scatter>),
    Assignment(Box<Assignment>),
    Block(Block),
}

impl Expression {
    pub(crate) fn compile(&self, symbols: &Symbols) -> Result<expressions::Expression, Error> {
        match self {
            Expression::Identifier(identifier) => {
                match symbols.var_entries.get(identifier) {
                    None => {
                        Err(Error::from(format!("Unknown variable {}.", identifier)))
                    }
                    Some(VarEntry::Uninitialized) => {
                        Ok(expressions::Expression::Identifier(identifier.clone()))
                    }
                    Some(VarEntry::Value(value)) => {
                        Ok(expressions::Expression::Value(value.clone()))
                    }
                }
            }
            Expression::Literal(literal) => {
                Ok(expressions::Expression::Value(literal.to_value()))
            }
            Expression::Binary(lhs, op, rhs) => {
                let eval_lhs = Box::new(lhs.compile(symbols)?);
                let eval_rhs = Box::new(rhs.compile(symbols)?);
                Ok(expressions::Expression::Binary(eval_lhs, *op,
                                                   eval_rhs))
            }
            Expression::Member(expression, identifier) => {
                let eval_expression = Box::new(expression.compile(symbols)?);
                Ok(expressions::Expression::Member(eval_expression,
                                                   identifier.clone()))
            }
            Expression::Call(callee, arguments) => {
                let eval_callee = Box::new(callee.compile(symbols)?);
                let mut eval_arguments = Vec::<eval::expressions::Assignment>::new();
                for argument in arguments {
                    eval_arguments.push(argument.compile(symbols)?);
                }
                Ok(expressions::Expression::Call(eval_callee, eval_arguments))
            }
            Expression::Scatter(scatter) => {
                let eval_iteration_lhs = scatter.iteration.lhs.clone();
                let eval_iteration_expression = scatter.iteration.rhs.compile(symbols)?;
                let symbols_scatter =
                        symbols.clone()
                            .with_var_uninitialized_entry(&eval_iteration_lhs);
                let eval_expression =
                    scatter.expression.compile(&symbols_scatter)?;
                let eval_iteration =
                    expressions::Iteration::new(eval_iteration_lhs, eval_iteration_expression);
                let eval_scatter =
                    expressions::Scatter::new(eval_iteration, eval_expression);
                Ok(expressions::Expression::Scatter(Box::new(eval_scatter)))
            }
            Expression::Assignment(assignment) => {
                let eval_assignment = assignment.compile(symbols)?;
                Ok(expressions::Expression::Assignment(Box::new(eval_assignment)))
            }
            Expression::Block(block) => {
                let eval_expressions =
                    compile_expressions(&block.expressions, symbols)?;
                let eval_block = expressions::Block::new(eval_expressions);
                Ok(expressions::Expression::Block(Box::new(eval_block)))
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
            Literal::String(string) => { format!("\"{}\"", string).fmt(f) }
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