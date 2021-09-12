use crate::util::error::Error;
use std::io::{BufReader, BufWriter, BufRead, Write};
use fs_err::File;

const PICK_COL: &str = "PICK";
const PICK_VALUE: &str = "1";

pub(crate) fn pick_records(input_file: &str, output_file: &str) -> Result<(), Error> {
    let reader = BufReader::new(File::open(input_file)?);
    let mut writer = BufWriter::new(File::create(output_file)?);
    let mut i_pick: usize = 0;
    for line_res in reader.lines() {
        let line = line_res?;
        if line.starts_with("##") {
            writeln!(&mut writer, "{}", &line)?;
        } else if line.starts_with('#') {
            if let Some((i, _)) =
            line.split('\t').enumerate().find(|(_, value)| {
                *value == PICK_COL
            }) {
                i_pick = i;
            }
            writeln!(&mut writer, "{}", &line)?;
        } else if let Some(value) = line.split('\t').nth(i_pick) {
            if value == PICK_VALUE {
                writeln!(&mut writer, "{}", &line)?;
            }
        }
    }
    Ok(())
}