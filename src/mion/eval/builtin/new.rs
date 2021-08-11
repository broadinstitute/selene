use crate::mion::eval::expressions::Function;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;
use crate::mion::eval::values::Value;
use crate::util::error::Error;

pub(crate) struct New {}

impl Function for New {
    fn id(&self) -> &str { "new" }

    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        Ok(Value::Object(args_map))
    }
}