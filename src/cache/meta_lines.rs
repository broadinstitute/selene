use std::io::Write;
use crate::util::error::Error;

pub(crate) fn chromosome_line(chromosomes: &[String]) -> String {
    let mut line = String::from("## Chromosomes: ");
    let mut is_first = true;
    for chromosome in chromosomes {
        if is_first {
            line.push_str(chromosome);
            is_first = false;
        } else {
            line.push_str(", ");
            line.push_str(chromosome);
        }
    }
    line
}

pub(crate) fn write_meta_lines(write: &mut impl Write, meta_lines: &[String])
                               -> Result<(), Error> {
    for meta_line in meta_lines {
        write.write_all(format!("{}\n", meta_line).as_bytes())?;
    }
    Ok(())
}
