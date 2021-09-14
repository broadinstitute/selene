use fs_err::File;
use std::io::{BufReader, Lines, BufRead};
use crate::util::error::Error;
use crate::tsv::util::{col_indices_from_header_line, extract_data_from_line};

struct ColIndices {
    i_pos: usize,
}

pub(crate) struct InputFile {
    i_cols: ColIndices,
    lines: Lines<BufReader<File>>,
}

pub(crate) struct HeadersAndInputFile {
    pub(crate) header_lines: Vec<String>,
    pub(crate) input_file: InputFile
}

pub(crate) struct PosLine {
    pub(crate) pos: u32,
    pub(crate) line: String,
}

pub(crate) struct InputFileConfig {
    file: String,
    pos_col: String,
}

impl InputFileConfig {
    pub(crate) fn new(file: String, pos_col: String) -> InputFileConfig {
        InputFileConfig { file, pos_col }
    }
}

impl InputFile {
    pub(crate) fn open(config: &InputFileConfig) -> Result<HeadersAndInputFile, Error> {
        let mut header_lines = Vec::<String>::new();
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
                        header_lines.push(line);
                        continue;
                    } else if line.starts_with('#') {
                        let cols = vec!(config.pos_col.as_str());
                        let i_cols_vec = col_indices_from_header_line(&line, &cols)?;
                        let i_pos = i_cols_vec[0];
                        let i_cols = ColIndices { i_pos };
                        let input_file = InputFile { i_cols, lines };
                        header_lines.push(line);
                        return Ok(HeadersAndInputFile { header_lines, input_file });
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

fn parse_data_line(line: String, i_cols: &[usize]) -> Result<PosLine, Error> {
    let parts = extract_data_from_line(&line, i_cols)?;
    let pos = parts[0].parse::<u32>()?;
    Ok(PosLine { pos, line })
}

impl Iterator for InputFile {
    type Item = Result<PosLine, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => {
                None
            }
            Some(Err(error)) => {
                Some(Err(Error::from(error)))
            }
            Some(Ok(line)) => {
                let i_cols = vec!(self.i_cols.i_pos);
                Some(parse_data_line(line, &i_cols))
            }
        }
    }
}
