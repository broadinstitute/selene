use crate::mion::eval::expressions::Function;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::values::Value;
use crate::util::error::Error;
use crate::mion::eval::builtin::utils::get_string_arg;
use crate::util::path_util::replace_file_name;

pub(crate) struct ReplaceFileName {}

const PATH_ARG: &str = "path";
const FILE_NAME_ARG: &str = "file_name";

impl Function for ReplaceFileName {
    fn id(&self) -> &str { "replace_file_name" }

    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        let path = get_string_arg(&args_map, PATH_ARG)?;
        let file_name = get_string_arg(&args_map, FILE_NAME_ARG)?;
        let new_path =
            replace_file_name(path.as_str(), file_name.as_str())?;
        Ok(Value::from(&new_path))
    }
}