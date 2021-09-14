use crate::util::error::Error;
use crate::tsv::id_pos_line::{InputFile, IdPosLine, InputFileConfig } ;

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
    fn match_last_against(&mut self, o_cache: &mut Cache) -> Option<(IdPosLine, IdPosLine)> {
        if let Some(last_record) = self.records.last() {
            let mut i_o_record_opt: Option<usize> = None;
            for (i, o_record) in o_cache.records.iter().enumerate() {
                if Cache::records_match(last_record, o_record) {
                    i_o_record_opt = Some(i);
                }
            }
            if let Some(i_o_record) = i_o_record_opt {
                let last_record = self.remove_record(self.records.len() - 1);
                let matching_o_record = o_cache.remove_record(i_o_record);
                Some((last_record, matching_o_record))
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct OutputFile {

}

impl OutputFile {
    fn new(_output_file: &str) -> Result<OutputFile, Error> {
        todo!()
    }
    fn write_joined(&mut self, record1: IdPosLine, record2: IdPosLine) -> Result<(), Error> {
        let _todo = record1.line;
        let _todo = record2.line;
        todo!()
    }
}

pub(crate) fn join(input_file_config1: &InputFileConfig, input_file_config2: &InputFileConfig,
                   output_file: &str)
                   -> Result<(), Error> {
    let input_file1 = InputFile::open(input_file_config1)?;
    let input_file2 = InputFile::open(input_file_config2)?;
    let mut cache1 = Cache::new(input_file1);
    let mut cache2 = Cache::new(input_file2);
    let mut output_file = OutputFile::new(output_file)?;
    while !cache1.is_exhausted() || !cache2.is_exhausted() {
        if cache1.pos_max <= cache2.pos_max {
            cache1.load_next()?;
            let matching_records_opt = cache1.match_last_against(&mut cache2);
            if let Some((record1, record2)) = matching_records_opt {
                output_file.write_joined(record1, record2)?;
                todo!( )
            };
        }
        todo!()
    }
    Ok(())
}