use crate::config::Config;
use crate::util::error::Error;
use crate::tools::{vep_output_transform, id_adapt};

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
        Config::AdaptIdPipe(adapt_id_config) => {
            id_adapt::adapt_id_pipe(&adapt_id_config.id_col)
        }
    }
}