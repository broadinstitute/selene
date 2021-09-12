use crate::mion::eval::expressions::Function;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::values::{Value, ObjectBuilder};
use crate::util::error::Error;
use crate::mion::eval::builtin::utils::get_string_arg;
use crate::tools::file_join;

pub(crate) struct JoinFiles {}

const INPUT_FILE1_ARG: &str = "input_file1";
const ID1_ARG: &str = "id1";
const INPUT_FILE2_ARG: &str = "input_file2";
const ID2_ARG: &str = "id2";
const OUTPUT_FILE_ARG: &str = "output_file";

impl Function for JoinFiles {
    fn id(&self) -> &str { "join_files" }

    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        let input_file1 = get_string_arg(&args_map, INPUT_FILE1_ARG)?;
        let id1 = get_string_arg(&args_map, ID1_ARG)?;
        let input_file2 = get_string_arg(&args_map, INPUT_FILE2_ARG)?;
        let id2 = get_string_arg(&args_map, ID2_ARG)?;
        let output_file = get_string_arg(&args_map, OUTPUT_FILE_ARG)?;
        file_join::join(&input_file1, &id1, &input_file2, &id2, &output_file)?;
        let object = ObjectBuilder::new()
            .with_string("output_file", &output_file)
            .into_object();
        Ok(object)
    }
}