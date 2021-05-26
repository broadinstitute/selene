use std::io::{Write, BufWriter};
use crate::util::error::Error;
use std::fs::File;

pub(crate) struct MissesFile {
    write: Box<dyn Write>,
}

impl MissesFile {
    pub(crate) fn from_file(out_file: String) -> Result<MissesFile, Error> {
        let write: Box<dyn Write> =
            Box::new(BufWriter::new(File::open(out_file)?));
        Ok(MissesFile { write })
    }
    pub(crate) fn from_stdout() -> MissesFile {
        let write: Box<dyn Write> = Box::new(std::io::stdout());
        MissesFile { write }
    }
}