use crate::mion::eval::expressions::Function;
use crate::mion::eval::values::Value;
use crate::util::error::Error;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;

pub(crate) struct Tabix {}

impl Function for Tabix {
    fn id(&self) -> &str { "tabix" }

    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        println!("And nothing else matters");
        Ok(Value::from(&String::from("Yeah")))
    }
}