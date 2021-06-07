use std::fs::File;
use crate::util::error::Error;
use std::io::{Write, BufWriter};
use crate::meta_lines;

pub(crate) struct Output {
    write: Box<dyn Write>,
}

fn write_header_line(write: &mut Box<dyn Write>, header_line: &str) -> Result<(), Error> {
    write.write_all(format!("{}\n", header_line).as_bytes())?;
    Ok(())
}

impl Output {
    pub(crate) fn from_file(out_file: String, header_line: &str, meta_lines: &[String])
        -> Result<Output, Error> {
        let mut write: Box<dyn Write> =
            Box::new(BufWriter::new(File::create(out_file)?));
        meta_lines::write_meta_lines(&mut write, &meta_lines)?;
        write_header_line(&mut write, &header_line)?;
        Ok(Output { write })
    }
    pub(crate) fn from_stdout(header_line: &str, meta_lines: &[String])
        -> Result<Output, Error> {
        let mut write: Box<dyn Write> = Box::new(std::io::stdout());
        meta_lines::write_meta_lines(&mut write, &meta_lines)?;
        write_header_line(&mut write, &header_line)?;
        Ok(Output { write })
    }
    pub(crate) fn write_line(&mut self, line: String) -> Result<(), Error> {
        self.write.write_all(line.as_bytes())?;
        Ok(())
    }
}