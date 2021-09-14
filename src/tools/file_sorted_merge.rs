use crate::util::error::Error;
use crate::tsv::pos_line::{PosLine, InputFile, InputFileConfig};

struct Cache {
    input_file: InputFile,
    record_opt: Option<PosLine>,
}

impl Cache {
    fn unpack_record(input_file: &mut InputFile) -> Result<Option<PosLine>, Error> {
        match input_file.next() {
            None => { Ok(None) }
            Some(record_res) => { Ok(Some(record_res?)) }
        }
    }
    fn new(mut input_file: InputFile) -> Result<Cache, Error> {
        let record_opt = Cache::unpack_record(&mut input_file)?;
        Ok(Cache { input_file, record_opt })
    }
    fn is_exhausted(&self) -> bool { self.record_opt.is_none() }
    fn get_pos(&self) -> Option<u32> {
        self.record_opt.as_ref().map(|record| { record.pos })
    }
    fn remove_record(&mut self) -> Result<Option<PosLine>, Error> {
        let record_opt = self.record_opt.take();
        self.record_opt = Cache::unpack_record(&mut self.input_file)?;
        Ok(record_opt)
    }
}

struct OutputFile {}

impl OutputFile {
    fn new(_output_file: &str) -> Result<OutputFile, Error> {
        todo!()
    }
}

pub(crate) fn merge(input_file_config1: &InputFileConfig, input_file_config2: &InputFileConfig,
                    output_file: &str)
                    -> Result<(), Error> {
    let input_file1 = InputFile::open(input_file_config1)?;
    let input_file2 = InputFile::open(input_file_config2)?;
    let mut cache1 = Cache::new(input_file1)?;
    let mut cache2 = Cache::new(input_file2)?;
    let mut output_file = OutputFile::new(output_file)?;
    while !cache1.is_exhausted() || !cache2.is_exhausted() {
        todo!()
    }
    Ok(())
}