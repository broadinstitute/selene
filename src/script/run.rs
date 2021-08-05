use crate::Error;
use crate::config::ScriptConfig;
use crate::mion::syntax::parser;
use std::fs;

pub(crate) fn run_script(script_config: ScriptConfig) -> Result<(), Error> {
    let script_string = fs::read_to_string(&script_config.script_file)?;
    println!("Life is ours, we live it our way");
    println!("Script file: {}", &script_config.script_file);
    println!("{}", &script_string);
    let script = parser::parse_script(script_string.as_str())?;
    println!("{}", &script);
    let compiled = script.compile()?;
    let optimized = compiled.optimize();
    let value = optimized.evaluate()?;
    println!("Value: {}", value);
    Ok(())
}