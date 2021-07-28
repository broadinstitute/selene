use nom::IResult;
use nom::error::{VerboseError, context};
use super::expressions::Expression;
use super::expressions::Literal;
use nom::combinator::recognize;
use nom::sequence::pair;
use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::multi::many0;
use nom::bytes::complete::tag;

type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub(crate) fn identifier(i: &str) -> ParseResult<Expression> {
    context(
        "identifier",
        recognize(
            pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )
        ))(i)
        .map(|(next_i, id_str)| (next_i, Expression::new_identifier(id_str)))
}

pub(crate) fn string_literal(i: &str) -> ParseResult<Literal> {
    unimplemented!()
}
