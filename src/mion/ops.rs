use std::fmt::{Display, Formatter};

pub(crate) enum BinOp {
    Plus,
    Minus,
    Times,
    By,
    Dot,
    LeftArrow,
    Assign,
}

pub(crate) enum BinOpGroup {
    PlusMinus,
    TimesBy,
    Dot,
    LeftArrow,
    Assign,
}

impl BinOp {
    pub fn symbol(&self) -> &str {
        match self {
            BinOp::Plus => { "+" }
            BinOp::Minus => { "-" }
            BinOp::Times => { "*" }
            BinOp::By => { "/" }
            BinOp::Dot => { "." }
            BinOp::LeftArrow => { "<-" }
            BinOp::Assign => { "=" }
        }
    }
    pub fn group(&self) -> BinOpGroup {
        match self {
            BinOp::Plus => { BinOpGroup::PlusMinus }
            BinOp::Minus => { BinOpGroup::PlusMinus }
            BinOp::Times => { BinOpGroup::TimesBy }
            BinOp::By => { BinOpGroup::TimesBy }
            BinOp::Dot => { BinOpGroup::Dot }
            BinOp::LeftArrow => { BinOpGroup::LeftArrow }
            BinOp::Assign => { BinOpGroup::Assign }
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str(self.symbol()) }
}

impl BinOpGroup {
    pub fn precedence(&self) -> u8 {
        match self {
            BinOpGroup::PlusMinus => { 2 }
            BinOpGroup::TimesBy => { 1 }
            BinOpGroup::Dot => { 0 }
            BinOpGroup::LeftArrow => { 3 }
            BinOpGroup::Assign => { 4 }
        }
    }
}