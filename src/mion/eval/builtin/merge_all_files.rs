use crate::mion::eval::expressions::Function;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::values::Value;
use crate::util::error::Error;
use crate::mion::eval::builtin::utils::{get_object_arg, get_array_arg};
use std::io::{BufWriter, Write, BufReader, Read, BufRead};
use std::fs::File;

pub(crate) struct MergeAllFiles {}

const FILE_LIST_ARG: &str = "file_list";
const SHARDS_ARG: &str = "shards";

struct Writers {
    output_files: HashMap<Identifier, String>,
    writers: HashMap<Identifier, BufWriter<File>>,
}

fn append_to_writer(input_file_name: &str, writer: &mut BufWriter<File>,
                    skip_header: bool)
    -> Result<(), Error> {
    let mut reader = BufReader::new(File::open(input_file_name)?);
    for line_res in reader.lines() {
        let line = line_res?;
        if !(skip_header && line.starts_with('#')) {
            writer.write_all(line.as_bytes())?;
            writer.write_all("\n".as_bytes())?;
        }
    }
    Ok(())
}

impl Writers {
    fn new(output_files: HashMap<Identifier, String>) -> Writers {
        let writers = HashMap::<Identifier, BufWriter<File>>::new();
        Writers { output_files, writers }
    }
    fn from_values(output_file_values: &HashMap<Identifier, Value>) -> Result<Writers, Error> {
        let mut output_files = HashMap::<Identifier, String>::new();
        for (identifier, value) in output_file_values.iter() {
            let value_string = value.as_string()?;
            output_files.insert(identifier.clone(), value_string);
        }
        Ok(Writers::new(output_files))
    }
    fn append_file(&mut self, identifier: &Identifier, file_name: &str) -> Result<(), Error> {
        match self.writers.get_mut(&identifier) {
            None => {
                let output_file_name =
                    self.output_files.get(&identifier).ok_or_else(|| {
                        Error::from(format!("No file name provided for output file {}",
                                            identifier))
                    })?;
                let mut writer =
                    BufWriter::new(File::create(output_file_name)?);
                append_to_writer(file_name, &mut writer, false)?;
                self.writers.insert(identifier.clone(), writer);
                Ok(())
            }
            Some(writer) => {
                append_to_writer(file_name, writer, true)
            }
        }
    }
}

impl Function for MergeAllFiles {
    fn id(&self) -> &str { "merge_all_files" }
    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        let file_list = get_object_arg(&args_map, &FILE_LIST_ARG)?;
        let shards = get_array_arg(&args_map, &SHARDS_ARG)?;
        let mut writers = Writers::from_values(file_list)?;
        for shard in shards {
            let shard_map = shard.as_map_ref()?;
            for (key, _) in file_list.iter() {
                let in_file_name_value =
                    shard_map.get(key)
                        .ok_or_else(|| {
                            Error::from(
                                format!("Missing file name for {}.", key))
                        })?;
                let in_file_name = in_file_name_value.as_string()?;
                writers.append_file(key, &in_file_name)?;
            }
        }
        Ok(Value::Unit)
    }
}