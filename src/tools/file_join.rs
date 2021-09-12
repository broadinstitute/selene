use crate::util::error::Error;
use fs_err::File;
use std::io::{BufReader, BufRead, Lines};

struct InputFile {
    i_id: usize,
    lines: Lines<BufReader<File>>,
}

struct IdLine {
    id: String,
    line: String,
}

impl InputFile {
    fn open(file: &str, id: &str) -> Result<InputFile, Error> {
        let mut lines = BufReader::new(File::open(file)?).lines();
        loop {
            match lines.next() {
                None => {
                    return Err(Error::from(
                        format!("Unexpected end of '{}' while parsing header lines", file
                        )));
                }
                Some(lines_res) => {
                    let line = lines_res?;
                    if line.starts_with("##") {
                        continue;
                    } else if line.starts_with('#') {
                        for (i, col) in line.split('\t').enumerate() {
                            if col == id {
                                let i_id = i;
                                return Ok(InputFile { i_id, lines });
                            }
                        }
                        return Err(
                            Error::from(format!("No column '{}' found in '{}'.", id, file))
                        );
                    } else {
                        return Err(Error::from(
                            format!("Unexpected end of '{}' while parsing header lines",
                                    file
                            )));
                    }
                }
            }
        }
    }
}

impl Iterator for InputFile {
    type Item = Result<IdLine, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => None,
            Some(Err(error)) => { Some(Err(Error::from(error))) }
            Some(Ok(line)) => {
                match line.split('\t').nth(self.i_id) {
                    None => { Some(Err(Error::from("Record has no id."))) }
                    Some(id) => { Some(Ok(IdLine { id: String::from(id), line })) }
                }
            }
        }
    }
}

pub(crate) fn join(input_file1: &str, id1: &str, input_file2: &str, id2: &str, output_file: &str)
                   -> Result<(), Error> {
    let input_file1 = InputFile::open(input_file1, id1)?;
    let input_file2 = InputFile::open(input_file2, id2)?;
    todo!()
}