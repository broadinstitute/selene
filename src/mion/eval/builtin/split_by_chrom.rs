use crate::mion::eval::expressions::{Function, Assignment};
use crate::mion::eval::values::Value;
use crate::util::error::Error;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;

pub(crate) struct SplitByChrom {}

impl Function for SplitByChrom {
    fn id(&self) -> &str { "split_by_chrom" }
    fn call(&self, args_map: HashMap<Identifier, Value>) -> Result<Value, Error> {
        println!("All these words I don't just say");
        Ok(Value::Int(42))
    }
}

