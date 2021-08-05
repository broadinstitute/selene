use crate::util::error::Error;
use super::symbols::{Symbols, SymbolsAndThing};
use super::values::Value;

trait Evaluatable {
    fn optimize(self, symbols: Symbols)
                -> SymbolsAndThing<Box<Self>> where Self: Sized {
        SymbolsAndThing::new(symbols, Box::new(self))
    }
    fn evaluate(&self, symbols: Symbols) -> SymbolsAndThing<Result<Value, Error>>;
}

pub(crate) struct Script {}

impl Script {
    pub(crate) fn new() -> Script { Script {} }
    pub(crate) fn optimize(self) -> Script { self }
    pub(crate) fn evaluate(&self) -> Result<Value, Error> { unimplemented!() }
}

