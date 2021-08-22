use crate::mion::eval::expressions::Function;
use crate::mion::eval::values::{Value, ObjectBuilder};
use crate::util::error::Error;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::builtin::utils::{get_string_arg, get_string_opt_arg};
use crate::config::{TabixInputConfig, TabixConfig};
use crate::cache::run::run_cache;

pub(crate) struct Tabix {}

const CACHE_FILE_ARG: &str = "cache_file";
const INDEX_FILE_ARG: &str = "index_file";
const INPUT_FILE_ARG: &str = "input_file";
const REGIONS_FILE_ARG: &str = "regions_file";
const COL_REF_ARG: &str = "col_ref";
const COL_ALT_ARG: &str = "col_alt";
const OUTPUT_FILE_ARG: &str = "output_file";
const MISSES_FILE_ARG: &str = "misses_file";

impl Function for Tabix {
    fn id(&self) -> &str { "tabix" }

    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        let cache_file = get_string_arg(&args_map, &CACHE_FILE_ARG)?;
        let index_file_opt = get_string_opt_arg(&args_map, &INDEX_FILE_ARG)?;
        let input_file = get_string_arg(&args_map, &INPUT_FILE_ARG)?;
        let regions_file_opt = get_string_opt_arg(&args_map, &REGIONS_FILE_ARG)?;
        let col_ref = get_string_arg(&args_map, &COL_REF_ARG)?;
        let col_alt = get_string_arg(&args_map, &COL_ALT_ARG)?;
        let output_file = get_string_arg(&args_map, &OUTPUT_FILE_ARG)?;
        let misses_file = get_string_arg(&args_map, &MISSES_FILE_ARG)?;
        let object = ObjectBuilder::new()
            .with_string("output_file", &output_file)
            .with_string("misses_file", &misses_file)
            .into_object();
        let input_config =
            TabixInputConfig::new(
                cache_file, index_file_opt, input_file, regions_file_opt, col_ref, col_alt
            );
        let config =
            TabixConfig::new(input_config, Some(misses_file),
                             Some(output_file));
        run_cache(config)?;
        Ok(object)
    }
}