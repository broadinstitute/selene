use crate::mion::eval::symbols::Symbols;
use crate::mion::eval::builtin::split_by_chrom::SplitByChrom;
use crate::mion::eval::builtin::tabix::Tabix;

pub(crate) fn predef_symbols() -> Symbols {
    Symbols::new()
        .with_function_entry(Box::new(SplitByChrom {}))
        .with_function_entry(Box::new(Tabix {}))
}
