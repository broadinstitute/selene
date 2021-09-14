use fs_err::File;
use std::io::{BufReader, Lines, BufRead};
use crate::util::error::Error;
use crate::tsv::util::{col_indices_from_header_line, extract_data_from_line};

struct ColIndices {
    i_id: usize,
    i_pos: usize,
}

pub(crate) struct InputFile {
    i_cols: ColIndices,
    lines: Lines<BufReader<File>>,
}

pub(crate) struct IdPosLine {
    pub(crate) id: String,
    pub(crate) pos: u32,
    pub(crate) line: String,
}

pub(crate) struct InputFileConfig {
    file: String,
    id_col: String,
    pos_col: String,
}

impl InputFileConfig {
    pub(crate) fn new(file: String, id_col: String, pos_col: String) -> InputFileConfig {
        InputFileConfig { file, id_col, pos_col }
    }
}

impl InputFile {
    pub(crate) fn open(config: &InputFileConfig) -> Result<InputFile, Error> {
        let mut lines =
            BufReader::new(File::open(&config.file)?).lines();
        loop {
            match lines.next() {
                None => {
                    return Err(Error::from(
                        format!("Unexpected end of '{}' while parsing header lines",
                                config.file)
                    ));
                }
                Some(lines_res) => {
                    let line = lines_res?;
                    if line.starts_with("##") {
                        continue;
                    } else if line.starts_with('#') {
                        let cols = vec!(config.id_col.as_str(), config.pos_col.as_str());
                        let i_cols_vec = col_indices_from_header_line(&line, &cols)?;
                        let i_id = i_cols_vec[0];
                        let i_pos = i_cols_vec[1];
                        let i_cols = ColIndices { i_id, i_pos };
                        return Ok(InputFile { i_cols, lines });
                    } else {
                        return Err(Error::from(
                            format!("Unexpected end of '{}' while parsing header lines",
                                    config.file
                            )));
                    }
                }
            }
        }
    }
}

fn parse_data_line(line: String, i_cols: &[usize]) -> Result<IdPosLine, Error> {
    let parts = extract_data_from_line(&line, i_cols)?;
    let id = String::from(parts[0]);
    let pos = parts[1].parse::<u32>()?;
    Ok(IdPosLine { id, pos, line })
}

impl Iterator for InputFile {
    type Item = Result<IdPosLine, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => {
                None
            }
            Some(Err(error)) => {
                Some(Err(Error::from(error)))
            }
            Some(Ok(line)) => {
                let i_cols = vec!(self.i_cols.i_id, self.i_cols.i_pos);
                Some(parse_data_line(line, &i_cols))
            }
        }
    }
}
