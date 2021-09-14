use crate::util::error::Error;
use fs_err::File;
use std::io::{BufReader, BufRead, Lines};
use crate::tsv::{col_indices_from_header_line, extract_data_from_line};

struct ColIndices {
    i_id: usize,
    i_pos: usize,
}

struct InputFile {
    i_cols: ColIndices,
    is_exhausted: bool,
    lines: Lines<BufReader<File>>,
}

struct IdPosLine {
    id: String,
    pos: u32,
    line: String,
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
    fn open(config: &InputFileConfig) -> Result<InputFile, Error> {
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
                        let is_exhausted = false;
                        return Ok(InputFile { i_cols, is_exhausted, lines });
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
                self.is_exhausted = true;
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

const POS_START: u32 = 0;
const POS_END: u32 = u32::MAX;

struct Cache {
    input_file: InputFile,
    records: Vec<IdPosLine>,
    pos_min: u32,
    pos_max: u32,
}

impl Cache {
    fn new(input_file: InputFile) -> Cache {
        let records = Vec::<IdPosLine>::new();
        let pos_min: u32 = POS_START;
        let pos_max: u32 = POS_START;
        Cache { input_file, records, pos_min, pos_max }
    }
}

impl Cache {
    fn is_exhausted(&self) -> bool { self.pos_min == POS_END }
    fn load_next(&mut self) -> Result<(), Error> {
        match self.input_file.next() {
            None => {
                self.pos_max = POS_END;
                Ok(())
            }
            Some(Ok(record)) => {
                self.pos_max = record.pos;
                self.records.push(record);
                Ok(())
            }
            Some(Err(error)) => {
                Err(error)
            }
        }
    }
    fn adjust_pos_min(&mut self) {
        match self.records.get(0) {
            None => { self.pos_min = self.pos_max }
            Some(record) => { self.pos_min = record.pos }
        }
    }
    fn remove_record(&mut self, i: usize) -> IdPosLine {
        let record = self.records.remove(i);
        self.adjust_pos_min();
        record
    }
    fn records_match(record1: &IdPosLine, record2: &IdPosLine) -> bool {
        record1.pos == record2.pos && record1.id == record2.id
    }
    fn match_last_against(&mut self, o_cache: &mut Cache) -> Option<String> {
        if let Some(last_record) = self.records.last() {
            let mut i_o_record_opt: Option<usize> = None;
            for (i, o_record) in o_cache.records.iter().enumerate() {
                if Cache::records_match(last_record, o_record) {
                    i_o_record_opt = Some(i);
                }
            }
            if let Some(i_o_record) = i_o_record_opt {
                todo!()
            }
        }
        todo!()
    }
}

pub(crate) fn join(input_file_config1: &InputFileConfig, input_file_config2: &InputFileConfig,
                   output_file: &str)
                   -> Result<(), Error> {
    let input_file1 = InputFile::open(input_file_config1)?;
    let input_file2 = InputFile::open(input_file_config2)?;
    let mut cache1 = Cache::new(input_file1);
    let cache2 = Cache::new(input_file2);
    while !cache1.is_exhausted() || !cache2.is_exhausted() {
        if cache1.pos_max <= cache2.pos_max {
            cache1.load_next()?;
        }
        todo!()
    }
    Ok(())
}