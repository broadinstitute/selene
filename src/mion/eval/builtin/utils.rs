use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::values::Value;
use crate::util::error::Error;

type ArgsMap = HashMap<Identifier, Value>;

pub(crate) fn get_string_arg(args_map: &ArgsMap, id: &str) -> Result<String, Error> {
    let value = args_map.get(&Identifier::from_str(id))
        .ok_or_else(||{ Error::from(format!("Missing argument {}", id))})?;
    value.as_string()
}