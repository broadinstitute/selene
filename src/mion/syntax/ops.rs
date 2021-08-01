use std::fmt::{Display, Formatter};
use crate::Error;

pub(crate) mod symbols {
    pub(crate) const DOT: &str = ".";
    pub(crate) const TIMES: &str = "*";
    pub(crate) const BY: &str = "/";
    pub(crate) const MODULO: &str = "%";
    pub(crate) const PLUS: &str = "+";
    pub(crate) const MINUS: &str = "-";
    pub(crate) const EQUAL: &str = "==";
    pub(crate) const NOT_EQUAL: &str = "!=";
    pub(crate) const LESS_THAN: &str = "<";
    pub(crate) const LESS_OR_EQUAL: &str = "<=";
    pub(crate) const GREATER_THAN: &str = ">";
    pub(crate) const GREATER_OR_EQUAL: &str = ">=";
    pub(crate) const AND: &str = "&&";
    pub(crate) const OR: &str = "||";
    pub(crate) const LEFT_ARROW: &str = "<-";
    pub(crate) const ASSIGN: &str = "=";
    pub(crate) const COMMA: &str = ",";
    pub(crate) const SEMICOLON: &str = ";";
    pub(crate) const OPEN_PARENS: &str = "(";
    pub(crate) const CLOSE_PARENS: &str = ")";
    pub(crate) const OPEN_BRACKETS: &str = "{";
    pub(crate) const CLOSE_BRACKETS: &str = "}";
}

#[derive(Clone)]
pub(crate) enum BinOp {
    Dot,
    Times,
    By,
    Modulo,
    Plus,
    Minus,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
    And,
    Or,
    LeftArrow,
    Assign,
    Comma,
    Semicolon,
}

pub(crate) enum BinOpGroup {
    Dot,
    TimesBy,
    PlusMinus,
    Compare,
    And,
    Or,
    LeftArrow,
    Assign,
    Punctuation,
}

impl BinOp {
    pub fn symbol(&self) -> &str {
        match self {
            BinOp::Dot => { symbols::DOT }
            BinOp::Times => { symbols::TIMES }
            BinOp::By => { symbols::BY }
            BinOp::Modulo => { symbols::MODULO }
            BinOp::Plus => { symbols::PLUS }
            BinOp::Minus => { symbols::MINUS }
            BinOp::Equal => { symbols::EQUAL }
            BinOp::NotEqual => { symbols::NOT_EQUAL }
            BinOp::LessThan => { symbols::LESS_THAN }
            BinOp::LessOrEqual => { symbols::LESS_OR_EQUAL }
            BinOp::GreaterThan => { symbols::GREATER_THAN }
            BinOp::GreaterOrEqual => { symbols::GREATER_OR_EQUAL }
            BinOp::And => { symbols::AND }
            BinOp::Or => { symbols::OR }
            BinOp::LeftArrow => { symbols::LEFT_ARROW }
            BinOp::Assign => { symbols::ASSIGN }
            BinOp::Comma => { symbols::COMMA }
            BinOp::Semicolon => { symbols::SEMICOLON }
        }
    }
    pub fn group(&self) -> BinOpGroup {
        match self {
            BinOp::Dot => { BinOpGroup::Dot }
            BinOp::Times => { BinOpGroup::TimesBy }
            BinOp::By => { BinOpGroup::TimesBy }
            BinOp::Modulo => { BinOpGroup::TimesBy }
            BinOp::Plus => { BinOpGroup::PlusMinus }
            BinOp::Minus => { BinOpGroup::PlusMinus }
            BinOp::Equal => { BinOpGroup::Compare }
            BinOp::NotEqual => { BinOpGroup::Compare }
            BinOp::LessThan => { BinOpGroup::Compare }
            BinOp::LessOrEqual => { BinOpGroup::Compare }
            BinOp::GreaterThan => { BinOpGroup::Compare }
            BinOp::GreaterOrEqual => { BinOpGroup::Compare }
            BinOp::And => { BinOpGroup::And }
            BinOp::Or => { BinOpGroup::Or }
            BinOp::LeftArrow => { BinOpGroup::LeftArrow }
            BinOp::Assign => { BinOpGroup::Assign }
            BinOp::Comma => { BinOpGroup::Punctuation }
            BinOp::Semicolon => { BinOpGroup::Punctuation }
        }
    }
    pub(crate) fn from_symbol(symbol: &str) -> Result<BinOp, Error> {
        match symbol {
            symbols::DOT => Ok(BinOp::Dot),
            symbols::TIMES => Ok(BinOp::Times),
            symbols::BY => Ok(BinOp::By),
            symbols::MODULO => Ok(BinOp::Modulo),
            symbols::PLUS => Ok(BinOp::Plus),
            symbols::MINUS => Ok(BinOp::Minus),
            symbols::EQUAL => Ok(BinOp::Equal),
            symbols::NOT_EQUAL => Ok(BinOp::NotEqual),
            symbols::LESS_THAN => Ok(BinOp::LessThan),
            symbols::LESS_OR_EQUAL => Ok(BinOp::LessOrEqual),
            symbols::GREATER_THAN => Ok(BinOp::GreaterThan),
            symbols::GREATER_OR_EQUAL => Ok(BinOp::GreaterOrEqual),
            symbols::AND => Ok(BinOp::And),
            symbols::OR => Ok(BinOp::Or),
            symbols::LEFT_ARROW => Ok(BinOp::LeftArrow),
            symbols::ASSIGN => Ok(BinOp::Assign),
            symbols::COMMA => Ok(BinOp::Comma),
            symbols::SEMICOLON => Ok(BinOp::Semicolon),
            _ => Err(
                Error::from(format!("Don't know a binary operator with symbol {}.",
                                    symbol))
            )
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str(self.symbol()) }
}

impl BinOpGroup {
    pub fn precedence(&self) -> u8 {
        match self {
            BinOpGroup::Dot => { 10 }
            BinOpGroup::TimesBy => { 9 }
            BinOpGroup::PlusMinus => { 8 }
            BinOpGroup::Compare => { 7 }
            BinOpGroup::And => { 6 }
            BinOpGroup::Or => { 5 }
            BinOpGroup::LeftArrow => { 4 }
            BinOpGroup::Assign => { 3 }
            BinOpGroup::Punctuation => { 2 }
        }
    }
}