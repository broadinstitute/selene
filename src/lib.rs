use crate::config::Config;
use crate::util::error::Error;
use crate::tools::vep_output_transform;

mod util;
mod config;
mod mion;
mod tabix;
mod cache;
mod script;
mod tools;
mod tsv;
mod genomics;

extern crate nom;

pub fn run() -> Result<(), Error> {
    let config = config::get_config()?;
    match config {
        Config::Tabix(tabix_config) => { cache::run::run_cache(tabix_config) }
        Config::Script(script_config) => { script::run::run_script(script_config) }
        Config::VepTransformPipe => { vep_output_transform::transform_vep_output_pipe() }
    }
}