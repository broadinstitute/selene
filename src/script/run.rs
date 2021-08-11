use crate::Error;
use crate::config::ScriptConfig;
use crate::mion::syntax::parser;
use std::fs;

pub(crate) fn run_script(script_config: ScriptConfig) -> Result<(), Error> {
    let script_string = fs::read_to_string(&script_config.script_file)?;
    println!("Life is ours, we live it our way");
    println!("Script file: {}", &script_config.script_file);
    println!("Begin script file");
    println!("{}", &script_string);
    println!("End script file");
    let script = parser::parse_script(script_string.as_str())?;
    println!("Begin parsed script");
    println!("{}", &script);
    println!("End parsed script");
    let compiled = script.compile()?;
    let optimized = compiled.optimize();
    let value = optimized.evaluate()?;
    println!("Begin final value");
    println!("Value: {}", value);
    println!("End final value");
    Ok(())
}