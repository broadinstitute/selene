use crate::mion::eval::symbols::Symbols;
use crate::mion::eval::expressions::Function;
use crate::mion::eval::values::Value;
use crate::util::error::Error;

pub(crate) fn predef_symbols() -> Symbols {
    Symbols::new()
        .with_function_entry(Box::new(SplitByChrom {}))
        .with_function_entry(Box::new(Tabix {}))
}

struct SplitByChrom {}

impl Function for SplitByChrom {
    fn id(&self) -> &str { "split_by_chrom" }
    fn call(&self, args: &[Value]) -> Result<Value, Error> {
        println!("All these words I don't just say");
        Ok(Value::Int(42))
    }
}

struct Tabix {}

impl Function for Tabix {
    fn id(&self) -> &str { "tabix" }

    fn call(&self, args: &[Value]) -> Result<Value, Error> {
        println!("And nothing else matters");
        Ok(Value::from(&String::from("Yeah")))
    }
}