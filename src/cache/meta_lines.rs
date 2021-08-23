use std::io::Write;
use crate::util::error::Error;

pub(crate) fn write_meta_lines(write: &mut impl Write, meta_lines: &[String])
                               -> Result<(), Error> {
    for meta_line in meta_lines {
        write.write_all(format!("{}\n", meta_line).as_bytes())?;
    }
    Ok(())
}
