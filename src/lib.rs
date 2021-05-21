mod util;
mod config;

use crate::util::error::Error;
use std::fs;
use bgzip::tabix::Tabix;

pub fn run() -> Result<(), Error> {
    let config = config::get_config()?;
    println!("data file: {}.", config.data_file);
    println!("index file: {}.", config.index_file);
    let tabix =
        Tabix::from_reader(&mut fs::File::open(&config.index_file)?)?;
    for name_raw in tabix.names {
        let name = String::from_utf8(name_raw)?;
        println!("Chromosome: {}", name)
    }
    Ok(())
}