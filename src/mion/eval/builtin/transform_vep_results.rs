use crate::mion::eval::expressions::Function;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::values::{Value, ObjectBuilder};
use crate::util::error::Error;
use crate::mion::eval::builtin::utils::get_string_arg;
use crate::tools::vep_output_transform::transform_vep_output_files;

pub(crate) struct TransformVepResults {}

const INPUT_FILE_ARG: &str = "input_file";
const OUTPUT_FILE_ARG: &str = "output_file";

impl Function for TransformVepResults {
    fn id(&self) -> &str { "transform_vep_results" }

    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        let input_file = get_string_arg(&args_map, INPUT_FILE_ARG)?;
        let output_file = get_string_arg(&args_map, OUTPUT_FILE_ARG)?;
        transform_vep_output_files(&input_file, &output_file)?;
        let object = ObjectBuilder::new()
            .with_string("input_file", &input_file)
            .with_string("output_file", &output_file)
            .into_object();
        Ok(object)
    }
}