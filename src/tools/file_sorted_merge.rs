use crate::util::error::Error;
use crate::tsv::pos_line::{PosLine, InputFile, InputFileConfig, HeadersAndInputFile};
use fs_err::File;
use std::io::{BufWriter, Write};

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

struct OutputFile {
    writer: BufWriter<File>
}

impl OutputFile {
    fn new(output_file: &str, header_lines: &[String]) -> Result<OutputFile, Error> {
        let mut writer = BufWriter::new(File::create(output_file)?);
        for header_line in header_lines {
            writeln!(writer, "{}", header_line)?;
        }
        Ok(OutputFile { writer })
    }
    fn write(&mut self, line: &str) -> Result<(), Error> {
        writeln!(self.writer, "{}", line)?;
        Ok(())
    }
}

const POS_MAX: u32 = u32::MAX;

pub(crate) fn merge(input_file_config1: &InputFileConfig, input_file_config2: &InputFileConfig,
                    output_file: &str)
                    -> Result<(), Error> {
    let HeadersAndInputFile { header_lines: header_lines1,
        input_file: input_file1} = InputFile::open(input_file_config1)?;
    let HeadersAndInputFile { input_file: input_file2, ..}
        = InputFile::open(input_file_config2)?;
    let mut cache1 = Cache::new(input_file1)?;
    let mut cache2 = Cache::new(input_file2)?;
    let mut output_file = OutputFile::new(output_file, &header_lines1)?;
    while !cache1.is_exhausted() || !cache2.is_exhausted() {
        let pos1 = cache1.get_pos().unwrap_or(POS_MAX);
        let pos2 = cache2.get_pos().unwrap_or(POS_MAX);
        if pos1 <= pos2 {
            if let Some(record) = cache1.remove_record()? {
                output_file.write(&record.line)?
            }
        }
        if pos2 <= pos1 {
            if let Some(record) = cache2.remove_record()? {
                output_file.write(&record.line)?
            }
        }
    }
    Ok(())
}