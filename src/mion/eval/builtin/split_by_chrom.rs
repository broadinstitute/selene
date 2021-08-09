use crate::mion::eval::expressions::Function;
use crate::mion::eval::values::Value;
use crate::util::error::Error;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::builtin::utils::get_string_arg;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fs::{File, create_dir};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub(crate) struct SplitByChrom {}

const INPUT_FILE_ARG: &str = "input_file";
const OUTPUT_FOLDER_ARG: &str = "output_folder";

impl Function for SplitByChrom {
    fn id(&self) -> &str { "split_by_chrom" }
    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        let input_file_string = get_string_arg(&args_map, &INPUT_FILE_ARG)?;
        let input_file = Path::new(&input_file_string);
        let output_folder_string = get_string_arg(&args_map, &OUTPUT_FOLDER_ARG)?;
        let output_folder = Path::new(&output_folder_string);
        let reader = BufReader::new(File::open(input_file)?);
        let mut header_lines = Vec::<String>::new();
        let mut writers = HashMap::<String, BufWriter<File>>::new();
        let mut output_file_paths = Vec::<Box<Path>>::new();
        for line_res in reader.lines() {
            let line = line_res?;
            if line.starts_with('#') {
                header_lines.push(line);
            } else {
                let mut fields = line.split('\t');
                if let Some(chrom) = fields.next() {
                    match writers.get_mut(chrom) {
                        None => {
                            let mut output_file_path_buf = PathBuf::from(output_folder);
                            output_file_path_buf.push(chrom);
                            create_dir(&output_file_path_buf)?;
                            let input_file_name =
                                input_file.file_name().ok_or_else(||{Error::from(
                                    format!("Cannot parse input file name '{}'.",
                                    input_file.as_os_str().to_string_lossy()))
                                })?;
                            output_file_path_buf.push(input_file_name);
                            let output_file_path = output_file_path_buf.into_boxed_path();
                            output_file_paths.push(output_file_path.clone());
                            let mut writer =
                                BufWriter::new(File::create(output_file_path)?);
                            for header_line in &header_lines {
                                writer.write_all(header_line.as_bytes())?;
                                writer.write_all("\n".as_bytes())?;
                            }
                            writer.write_all(line.as_bytes())?;
                            writer.write_all("\n".as_bytes())?;
                            writers.insert(chrom.to_string(), writer);
                        }
                        Some(writer) => {
                            writer.write_all(line.as_bytes())?;
                            writer.write_all("\n".as_bytes())?;
                        }
                    }
                } else {
                    println!("Missing chrom field for line: {}", line)
                }
            }
        }
        for (_, writer) in writers.iter_mut() {
            writer.flush()?;
        }
        let mut output_file_name_values = Vec::<Value>::new();
        for output_file_path in output_file_paths {
            let output_file_name = String::from(output_file_path.to_string_lossy());
            let output_file_name_value = Value::from(&output_file_name);
            output_file_name_values.push(output_file_name_value);
        }
        Ok(Value::Array(Arc::new(output_file_name_values)))
    }
}

