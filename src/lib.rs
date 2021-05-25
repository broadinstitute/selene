mod util;
mod config;
mod input;
mod variant;

use crate::util::error::Error;
use std::fs;
use bgzip::tabix::Tabix;
use crate::input::Input;

pub fn run() -> Result<(), Error> {
    let config = config::get_config()?;
    println!("data file: {}.", config.data_file);
    println!("index file: {}.", config.index_file);
    println!("input file: {}.", config.input_file);
    let output_file = match config.output_file_opt {
        Some(output_file) => output_file,
        None => String::from("STDOUT")
    };
    println!("output file: {}.", output_file);
    let tabix =
        Tabix::from_reader(&mut fs::File::open(&config.index_file)?)?;
    let input = Input::from_file(&config.input_file)?;
    for name_raw in tabix.names {
        let name = String::from_utf8(name_raw)?;
        println!("Chromosome: {}", name)
    }
    for variant in input.variants().take(10) {
        println!("{}", variant.canonical_id());
    }
    // BGZFReader::bgzf_seek();
    Ok(())
}