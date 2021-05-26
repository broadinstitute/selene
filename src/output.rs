use std::fs::File;
use crate::util::error::Error;
use std::io::{Write, BufWriter};

pub(crate) struct Output {
    write: Box<dyn Write>,
}

impl Output {
    pub(crate) fn from_file(out_file: String) -> Result<Output, Error> {
        let write: Box<dyn Write> =
            Box::new(BufWriter::new(File::open(out_file)?));
        Ok(Output { write })
    }
    pub(crate) fn from_stdout() -> Output {
        let write: Box<dyn Write> = Box::new(std::io::stdout());
        Output { write }
    }
}