pub(crate) struct Symbols {}

pub(crate) struct SymbolsAndThing<T> {
    symbols: Symbols,
    thing: T,
}

impl<T> SymbolsAndThing<T> {
    pub(crate) fn new(symbols: Symbols, thing: T) -> SymbolsAndThing<T> {
        SymbolsAndThing { symbols, thing }
    }
}