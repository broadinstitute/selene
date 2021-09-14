use crate::mion::eval::expressions::Function;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::values::{Value, ObjectBuilder};
use crate::util::error::Error;
use crate::mion::eval::builtin::utils::get_string_arg;
use crate::tools::file_sorted_merge;
use crate::tsv::pos_line::InputFileConfig;

pub(crate) struct MergeSortedFiles {}

const INPUT_FILE1_ARG: &str = "input_file1";
const POS_COL1_ARG: &str = "pos_col1";
const INPUT_FILE2_ARG: &str = "input_file2";
const POS_COL2_ARG: &str = "pos_col2";
const OUTPUT_FILE_ARG: &str = "output_file";

impl Function for MergeSortedFiles {
    fn id(&self) -> &str { "merge_sorted_files" }

    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        let input_file1 = get_string_arg(&args_map, INPUT_FILE1_ARG)?;
        let pos_col1 = get_string_arg(&args_map, POS_COL1_ARG)?;
        let input_file_config1 =
            InputFileConfig::new(input_file1, pos_col1);
        let input_file2 = get_string_arg(&args_map, INPUT_FILE2_ARG)?;
        let pos_col2 = get_string_arg(&args_map, POS_COL2_ARG)?;
        let input_file_config2 =
            InputFileConfig::new(input_file2, pos_col2);
        let output_file = get_string_arg(&args_map, OUTPUT_FILE_ARG)?;
        file_sorted_merge::merge(&input_file_config1, &input_file_config2, &output_file)?;
        let object = ObjectBuilder::new()
            .with_string("output_file", &output_file)
            .into_object();
        Ok(object)
    }
}