use std::fmt::{Display, Formatter};

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
}

impl BinOp {
    pub fn symbol(&self) -> &str {
        match self {
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
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str(self.symbol()) }
}
