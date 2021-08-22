use crate::mion::eval::expressions::Function;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::values::{Value, ObjectBuilder};
use crate::util::error::Error;
use crate::tools::vep::{run_vep, VepArgs, VepSetupArgs};
use crate::mion::eval::builtin::utils::get_string_arg;

const VEP_CMD_ARG: &str = "vep_cmd";
const INPUT_FILE_ARG: &str = "input_file";
const FASTA_FILE_ARG: &str = "fasta_file";
const CACHE_DIR_ARG: &str = "cache_dir";
const PLUGINS_DIR_ARG: &str = "plugins_dir";
const DBNSFP_ARG: &str = "dbnsfp";
const OUTPUT_FILE_ARG: &str = "output_file";
const WARNINGS_FILE_ARG: &str = "warnings_file";

pub(crate) struct Vep {}

impl Function for Vep {
    fn id(&self) -> &str { "vep" }

    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        let vep_cmd = get_string_arg(&args_map, VEP_CMD_ARG)?;
        let input_file = get_string_arg(&args_map, INPUT_FILE_ARG)?;
        let fasta_file = get_string_arg(&args_map, FASTA_FILE_ARG)?;
        let cache_dir = get_string_arg(&args_map, CACHE_DIR_ARG)?;
        let plugins_dir = get_string_arg(&args_map, PLUGINS_DIR_ARG)?;
        let dbnsfp = get_string_arg(&args_map, DBNSFP_ARG)?;
        let output_file = get_string_arg(&args_map, OUTPUT_FILE_ARG)?;
        let warnings_file = get_string_arg(&args_map, WARNINGS_FILE_ARG)?;
        let object = ObjectBuilder::new()
            .with_string("output_file", &output_file)
            .with_string("warnings_file", &warnings_file)
            .into_object();
        let vep_setup_args =
            VepSetupArgs::new(vep_cmd, fasta_file, cache_dir, plugins_dir, dbnsfp);
        let vep_args =
            VepArgs::new(input_file, output_file, warnings_file, vep_setup_args);
        run_vep(vep_args)?;
        Ok(object)
    }
}