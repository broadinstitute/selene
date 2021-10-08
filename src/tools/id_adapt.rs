use crate::util::error::Error;
use std::io;
use std::io::{BufReader, BufRead, BufWriter, Read, Write};

pub(crate) fn adapt_id_pipe(id_col: &str) -> Result<(), Error> {
    let reader = BufReader::new(io::stdin());
    let writer = BufWriter::new(io::stdout());
    adapt_id(reader, writer, id_col)
}

pub(crate) fn adapt_id<I, O>(reader: BufReader<I>, mut writer: BufWriter<O>, id_col: &str)
                             -> Result<(), Error>
    where I: Read, O: Write {
    const I_DUMMY: usize = 0;
    let mut i_id: usize = I_DUMMY;
    for line_res in reader.lines() {
        let mut line = line_res?;
        if line.starts_with("##") {
            writeln!(&mut writer, "{}", &line)?;
        } else if let Some(line_stripped) = line.strip_prefix('#') {
            let mut i_id_opt: Option<usize> = None;
            for (i, column) in line_stripped.split('\t').enumerate() {
                if column == id_col { i_id_opt = Some(i) }
                if i_id_opt.is_some() { break; }
            }
            i_id = i_id_opt.ok_or_else(|| { format!("Missing column {}", id_col) })?;
            writeln!(&mut writer, "#{}", &line_stripped)?;
        } else {
            let bytes = unsafe { line.as_bytes_mut() };
            let mut pos: usize = 0;
            let mut i: usize = 0;
            while pos < bytes.len() && i < i_id {
                if bytes[pos] == b'\t' {
                    i += 1;
                }
                pos += 1;
            }
            if i == i_id {
                let mut i_div: u8 = 0;
                while pos < bytes.len() {
                    let byte = bytes[pos];
                    if byte == b'_' || byte == b'/' || byte == b':' {
                        match i_div {
                            0 => { bytes[pos] = b':' }
                            1 => { bytes[pos] = b'_' }
                            2 => { bytes[pos] = b'/' }
                            _ => { break }
                        }
                        i_div += 1;
                    } else if byte == b'\t' {
                        break
                    }
                    pos += 1;
                }
            }
            let line_new = std::str::from_utf8(bytes)?;
            writeln!(writer, "{}", line_new)?;
        }
    }
    Ok(())
}