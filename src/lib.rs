use crate::config::Config;
use crate::util::error::Error;

mod util;
mod config;
mod variant;
mod mion;
mod tabix;
mod cache;
mod script;
mod tools;

extern crate nom;

pub fn run() -> Result<(), Error> {
    let config = config::get_config()?;
    match config {
        Config::Tabix(tabix_config) => { cache::run::run_cache(tabix_config) }
        Config::Script(script_config) => { script::run::run_script(script_config) }
    }
}