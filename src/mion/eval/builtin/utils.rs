use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::values::Value;
use crate::util::error::Error;

type ArgsMap = HashMap<Identifier, Value>;

pub(crate) fn get_string_arg(args_map: &ArgsMap, id: &str) -> Result<String, Error> {
    let value = args_map.get(&Identifier::from_str(id))
        .ok_or_else(|| { Error::from(format!("Missing argument {}", id)) })?;
    value.as_string()
}

pub(crate) fn get_string_opt_arg(args_map: &ArgsMap, id: &str) -> Result<Option<String>, Error> {
    match args_map.get(&Identifier::from_str(id)) {
        None => { Ok(None) }
        Some(value) => { Ok(Some(value.as_string()?)) }
    }
}

pub(crate) fn get_object_arg<'a>(args_map: &'a ArgsMap, id: &str)
    -> Result<&'a HashMap<Identifier, Value>, Error> {
    let value = args_map.get(&Identifier::from_str(id))
        .ok_or_else(|| { Error::from(format!("Missing argument {}", id)) })?;
    value.as_map_ref()
}

pub(crate) fn get_array_arg<'a>(args_map: &'a ArgsMap, id: &str)
                                 -> Result<&'a Vec<Value>, Error> {
    let value = args_map.get(&Identifier::from_str(id))
        .ok_or_else(|| { Error::from(format!("Missing argument {}", id)) })?;
    value.as_vec_ref()
}
