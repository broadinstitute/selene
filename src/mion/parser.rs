use nom::IResult;
use nom::error::{VerboseError, context};
use super::expressions::Expression;
use super::expressions::Literal;
use nom::combinator::{recognize, opt, map_res};
use nom::sequence::pair;
use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1, one_of};
use nom::multi::{many0, many1};
use nom::bytes::complete::{tag, take_while1};
use crate::mion::string;
use nom::number::complete::double;
use nom::Parser;
use nom::character::is_digit;

type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub(crate) trait MParser<'a, O>: Parser<&'a str, O, VerboseError<&'a str>> {}

impl<'a, T, O> MParser<'a, O> for T where T: Parser<&'a str, O, VerboseError<&'a str>> {}

pub(crate) fn identifier<'a>() -> impl MParser<'a, Expression> {
    context(
        "identifier",
        recognize(
            pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )
        )).map(|name| Expression::new_identifier(name))
}

pub(crate) fn string_literal<'a>() -> impl MParser<'a, Literal> {
    context("string literal", string::parse_string).map(|string| Literal::String(string))
}

pub(crate) fn float_literal<'a>() -> impl MParser<'a, Literal> {
    context("float literal", double).map(|x| { Literal::Float(x) })
}

pub(crate) fn integer_literal<'a>() -> impl MParser<'a, Literal> {
    map_res(recognize(pair(opt(alt((tag("-"), tag("+")))),
                           many1(one_of("0123456789")))),
            |int_str: &str| { int_str.parse::<i64>() })
        .map(|i| { Literal::Int(i) })
}

pub(crate) fn integer<'a>()  -> impl MParser<'a, Literal> {
    alt((string_literal(), float_literal(), integer_literal()))
}


