use nom::IResult;
use nom::error::{VerboseError, context};
use super::expressions::Expression;
use super::expressions::Literal;
use nom::combinator::{recognize, opt, map_res, value};
use nom::sequence::{pair, tuple};
use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1, one_of, none_of};
use nom::multi::{many0, many1};
use nom::bytes::complete::tag;
use crate::mion::string;
use nom::number::complete::double;
use nom::Parser;
use crate::mion::ops::{BinOp, symbols};
use crate::mion::expressions::Identifier;

pub(crate) trait MParser<'a, O>: Parser<&'a str, O, VerboseError<&'a str>> {}

impl<'a, T, O> MParser<'a, O> for T where T: Parser<&'a str, O, VerboseError<&'a str>> {}

pub(crate) fn identifier<'a>() -> impl MParser<'a, Identifier> {
    context(
        "identifier",
        recognize(
            pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )
        )).map(String::from).map(Identifier::new)
}

pub(crate) fn string_literal<'a>() -> impl MParser<'a, Literal> {
    context("string literal", string::parse_string).map(Literal::String)
}

pub(crate) fn float_literal<'a>() -> impl MParser<'a, Literal> {
    context("float literal", double).map(|x| { Literal::Float(x) })
}

pub(crate) fn integer_literal<'a>() -> impl MParser<'a, Literal> {
    context("integer literal",
            map_res(recognize(pair(opt(alt((tag("-"), tag("+")))),
                                   many1(one_of("0123456789")))),
                    |int_str: &str| { int_str.parse::<i64>() })
                .map(|i| { Literal::Int(i) }),
    )
}

pub(crate) fn literal<'a>() -> impl MParser<'a, Literal> {
    context("literal", alt((string_literal(), float_literal(), integer_literal())))
}

pub(crate) fn comment<'a>() -> impl MParser<'a, ()> {
    context("comment",
            value((), pair(tag("//"), none_of("\r\n"))),
    )
}

pub(crate) fn whitespace<'a>() -> impl MParser<'a, ()> {
    context("whitespace, including comments",
            value((),
                  many0(alt((value((), one_of(" \t\n\r")), comment()))
                  ),
            ),
    )
}

pub(crate) fn atomic<'a>() -> impl MParser<'a, Expression> {
    context("atomic expression",
            alt(
                (identifier().map(Expression::Identifier),
                 literal().map(Expression::Literal))
            ),
    )
}

pub(crate) fn member_selection<'a>() -> impl MParser<'a, Expression> {
    context("member selection",
            pair(
                atomic(),
                many0(
                    tuple((whitespace(), tag(symbols::DOT), whitespace(), identifier()))
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
    )
}

pub(crate) fn function_call<'a>() -> impl MParser<'a, Expression> {
    context("function call",
            pair(
                member_selection(),
                opt(
                    tuple((
                        whitespace(),
                        tag(symbols::OPEN_PARENS),
                        opt(tuple((
                            whitespace(),
                            member_selection(),
                            many0(tuple((
                                whitespace(),
                                tag(symbols::COMMA),
                                whitespace(),
                                member_selection()
                            )))
                        ))),
                        whitespace(),
                        tag(symbols::CLOSE_PARENS)
                    ))
                ),
            ).map(|parsed|{
                let (callee, args_in_parens_opt) = parsed;
                let mut function_call: Expression = callee;
                if let Some(args_in_parens) = args_in_parens_opt {
                    let (_, _, arg_list_opt, _, _) = args_in_parens;
                    if let Some(arg_list) = arg_list_opt {
                        let (_, arg0, args_remainder) = arg_list;
                        let mut args = Vec::<Expression>::new();
                        args.push(arg0);
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
    )
}



