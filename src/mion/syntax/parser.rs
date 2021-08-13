use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, none_of, one_of};
use nom::combinator::{map_res, opt, recognize, value, map};
use nom::error::{context, VerboseError};
use nom::IResult;
use nom::multi::{many0, many1};
use nom::number::complete::double;
use nom::Parser;
use nom::sequence::{pair, tuple, delimited};
use crate::Error;

use crate::mion::syntax::expressions::{Expression, Iteration, Assignment, Scatter, Block, Script};
use crate::mion::syntax::expressions::Literal;
use crate::mion::syntax::ops::{BinOp, symbols};
use crate::mion::syntax::string;
use crate::mion::eval::identifier::Identifier;

type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub(crate) fn identifier(i: &str) -> ParseResult<Identifier> {
    context(
        "identifier",
        map(map(recognize(
            pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
                String::from,
        ), Identifier::new))(i)
}

pub(crate) fn string_literal(i: &str) -> ParseResult<Literal> {
    context("string literal", map(string::parse_string, Literal::String))(i)
}

pub(crate) fn float_literal(i: &str) -> ParseResult<Literal> {
    context("float literal", map(double, Literal::Float))(i)
}

pub(crate) fn integer_literal(i: &str) -> ParseResult<Literal> {
    context("integer literal",
            map_res(recognize(pair(opt(alt((tag("-"), tag("+")))),
                                   many1(one_of("0123456789")))),
                    |int_str: &str| { int_str.parse::<i64>() })
                .map(|i| { Literal::Int(i) }),
    )(i)
}

pub(crate) fn literal(i: &str) -> ParseResult<Literal> {
    context("literal", alt((integer_literal, float_literal, string_literal)))(i)
}

pub(crate) fn comment(i: &str) -> ParseResult<()> {
    context("comment",
            value((), pair(tag("//"), none_of("\r\n"))),
    )(i)
}

pub(crate) fn whitespace(i: &str) -> ParseResult<()> {
    context("whitespace, including comments",
            value((),
                  many0(alt((value((), one_of(" \t\n\r")), comment))
                  ),
            ),
    )(i)
}

pub(crate) fn atomic(i: &str) -> ParseResult<Expression> {
    context("atomic expression",
            alt(
                (identifier.map(Expression::Identifier),
                 literal.map(Expression::Literal))
            ),
    )(i)
}

pub(crate) fn member_selection(i: &str) -> ParseResult<Expression> {
    context("member selection",
            pair(
                atomic,
                many0(
                    tuple((whitespace, tag(symbols::DOT), whitespace, identifier))
                ),
            ).map(|parsed| {
                let (expr0, member_selection_parts) = parsed;
                let mut expr: Expression = expr0;
                for member_selection_part in member_selection_parts {
                    let (_, _, _, identifier) = member_selection_part;
                    expr = Expression::Member(Box::new(expr), identifier);
                }
                expr
            }),
    )(i)
}

pub(crate) fn function_call(i: &str) -> ParseResult<Expression> {
    context("function call",
            pair(
                member_selection,
                opt(
                    tuple((
                        whitespace,
                        tag(symbols::OPEN_PARENS),
                        opt(tuple((
                            whitespace,
                            assignment,
                            many0(tuple((
                                whitespace,
                                tag(symbols::COMMA),
                                whitespace,
                                assignment
                            )))
                        ))),
                        whitespace,
                        tag(symbols::CLOSE_PARENS)
                    ))
                ),
            ).map(|parsed| {
                let (callee, args_in_parens_opt) = parsed;
                let mut function_call: Expression = callee;
                if let Some(args_in_parens) = args_in_parens_opt {
                    let (_, _, arg_list_opt, _, _) = args_in_parens;
                    if let Some(arg_list) = arg_list_opt {
                        let (_, arg0, args_remainder) = arg_list;
                        let mut args = vec!(arg0);
                        for arg_part in args_remainder {
                            let (_, _, _, arg) = arg_part;
                            args.push(arg);
                        }
                        function_call =
                            Expression::Call(Box::new(function_call),
                                             args)
                    } else {
                        function_call =
                            Expression::Call(Box::new(function_call),
                                             Vec::new())
                    }
                }
                function_call
            }),
    )(i)
}

pub(crate) fn product(i: &str) -> ParseResult<Expression> {
    context("product",
            pair(function_call,
                 many0(tuple((
                     whitespace,
                     alt((
                         value(BinOp::Times, tag(symbols::TIMES)),
                         value(BinOp::By, tag(symbols::BY)),
                         value(BinOp::Modulo, tag(symbols::MODULO))
                     )),
                     whitespace,
                     function_call
                 ))),
            ).map(build_bin_ops_chain),
    )(i)
}

pub(crate) fn sum(i: &str) -> ParseResult<Expression> {
    context("sum",
            pair(product,
                 many0(tuple((
                     whitespace,
                     alt((
                         value(BinOp::Plus, tag(symbols::PLUS)),
                         value(BinOp::Minus, tag(symbols::MINUS))
                     )),
                     whitespace,
                     product
                 ))),
            ).map(build_bin_ops_chain),
    )(i)
}

pub(crate) fn comparison(i: &str) -> ParseResult<Expression> {
    context("comparison",
            tuple((
                sum,
                whitespace,
                alt((
                    value(BinOp::Equal, tag(symbols::EQUAL)),
                    value(BinOp::NotEqual, tag(symbols::NOT_EQUAL)),
                    value(BinOp::LessThan, tag(symbols::LESS_THAN)),
                    value(BinOp::LessOrEqual, tag(symbols::LESS_OR_EQUAL)),
                    value(BinOp::GreaterThan, tag(symbols::GREATER_THAN)),
                    value(BinOp::GreaterOrEqual, tag(symbols::GREATER_OR_EQUAL))
                )),
                whitespace,
                sum,
            )).map(|parsed| {
                let (lhs, _, op, _, rhs) = parsed;
                Expression::Binary(Box::new(lhs), op,
                                   Box::new(rhs))
            }),
    )(i)
}

pub(crate) fn conjunction(i: &str) -> ParseResult<Expression> {
    context("conjunction",
            pair(alt((comparison, function_call)),
                 many0(tuple((
                     whitespace,
                     value(BinOp::And, tag(symbols::AND)),
                     whitespace,
                     alt((comparison, function_call))
                 ))),
            ).map(build_bin_ops_chain),
    )(i)
}

pub(crate) fn disjunction(i: &str) -> ParseResult<Expression> {
    context("disjunction",
            pair(conjunction,
                 many0(tuple((
                     whitespace,
                     value(BinOp::Or, tag(symbols::OR)),
                     whitespace,
                     conjunction
                 ))),
            ).map(build_bin_ops_chain),
    )(i)
}

fn build_bin_ops_chain(parsed: (Expression, Vec<((), BinOp, (), Expression)>)) -> Expression {
    let (term0, remainder) = parsed;
    let mut chain = term0;
    for part in remainder {
        let (_, op, _, term) = part;
        chain =
            Expression::Binary(Box::new(chain), op,
                               Box::new(term))
    }
    chain
}

pub(crate) fn iteration(i: &str) -> ParseResult<Iteration> {
    context("iteration",
            tuple((
                identifier, whitespace, tag(symbols::LEFT_ARROW), whitespace, sum
            )).map(|parsed| {
                let (lhs, _, _, _, rhs) = parsed;
                Iteration { lhs, rhs }
            }),
    )(i)
}

pub(crate) fn assignment(i: &str) -> ParseResult<Assignment> {
    context("assignment",
            tuple((
                identifier, whitespace, tag(symbols::ASSIGN), whitespace, expression
            )).map(|parsed| {
                let (lhs, _, _, _, rhs) = parsed;
                Assignment { lhs, rhs }
            }),
    )(i)
}

pub(crate) fn expression(i: &str) -> ParseResult<Expression> {
    alt((
        assignment.map(|assignment| Expression::Assignment(Box::new(assignment))),
        scatter.map(|scatter| Expression::Scatter(Box::new(scatter))),
        block.map(Expression::Block),
        disjunction,
        sum,
    ))(i)
}

pub(crate) fn scatter(i: &str) -> ParseResult<Scatter> {
    context("scatter",
            tuple((
                tag(symbols::OPEN_PARENS),
                whitespace,
                iteration,
                whitespace,
                tag(symbols::CLOSE_PARENS),
                whitespace,
                expression
            )).map(|parsed| {
                let (_, _, iteration, _, _, _, expression0) = parsed;
                let expression = Box::<Expression>::new(expression0);
                Scatter { iteration, expression }
            }),
    )(i)
}

pub(crate) fn expressions(i: &str) -> ParseResult<Vec<Expression>> {
    println!("expressions: {}", i);
    context("expressions",
            tuple((
                expression,
                whitespace,
                tag(symbols::SEMICOLON),
                many0(
                    tuple((
                        whitespace,
                        expression,
                        whitespace,
                        tag(symbols::SEMICOLON)
                    ))
                )
            )).map(|parsed| {
                let mut expressions = Vec::<Expression>::new();
                let (expression0, _, _, parsed_more) = parsed;
                expressions.push(expression0);
                for (_, expression, _, _) in parsed_more {
                    expressions.push(expression)
                }
                expressions
            }),
    )(i)
}

pub(crate) fn block(i: &str) -> ParseResult<Block> {
    context("block",
            delimited(
                pair(tag(symbols::OPEN_BRACKETS), whitespace),
                expressions,
                pair(whitespace, tag(symbols::CLOSE_BRACKETS)),
            ).map(|expressions| Block { expressions }),
    )(i)
}

pub(crate) fn script(i: &str) -> ParseResult<Script> {
    context("script", expressions.map(Script::new))(i)
}

pub(crate) fn parse_script(string: &str) -> Result<Script, Error> {
    Ok(script(string.trim())?.1)
}
