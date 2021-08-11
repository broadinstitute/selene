use crate::mion::eval::symbols::Symbols;
use crate::mion::eval::builtin::split_by_chrom::SplitByChrom;
use crate::mion::eval::builtin::tabix::Tabix;
use crate::mion::eval::builtin::new::New;
use crate::mion::eval::builtin::merge_all_files::MergeAllFiles;

pub(crate) fn predef_symbols() -> Symbols {
    Symbols::new()
        .with_function_entry(Box::new(SplitByChrom {}))
        .with_function_entry(Box::new(Tabix {}))
        .with_function_entry(Box::new(New {}))
        .with_function_entry(Box::new(MergeAllFiles {}))
}
