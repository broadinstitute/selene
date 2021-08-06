use crate::mion::eval::eval::Expression;
use crate::util::error::Error;
use crate::mion::eval::values::Value;
use crate::mion::syntax::expressions::Identifier;

pub(crate) struct Symbols {
    pub(crate) var_entries: VarEntries
}

pub(crate) enum VarEntry {
    Uninitialized,
    Value(Value)
}

pub(crate) enum VarEntries {
    Nil,
    Entry(Box<VarEntries>, Identifier, VarEntry)
}

impl VarEntries {
    pub(crate) fn get(&self, identifier: &Identifier) -> Option<&VarEntry> {
        match self {
            VarEntries::Nil => { None }
            VarEntries::Entry(parent, entry_identifier, entry) => {
                if identifier == *entry_identifier {
                    Some(entry)
                } else {
                    parent.get(identifier)
                }
            }
        }
    }
}

pub(crate) struct SymbolsExpressionResult {
    symbols: Symbols,
    expression_result: Result<Expression, Error>,
}

impl SymbolsExpressionResult {
    pub(crate) fn from_expression(symbols: Symbols, expression: Expression)
                                  -> SymbolsExpressionResult {
        let expression_result = Ok(expression);
        SymbolsExpressionResult { symbols, expression_result }
    }
    pub(crate) fn from_error(symbols: Symbols, error: Error) -> SymbolsExpressionResult {
        let expression_result = Err(error);
        SymbolsExpressionResult { symbols, expression_result }
    }
}
