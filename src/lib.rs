mod util;
mod config;
mod input;
mod variant;
mod output;
mod misses;
mod join;
mod tabix;

use crate::util::error::Error;
use std::fs::File;
use bgzip::tabix::Tabix;
use crate::input::Input;
use bgzip::BGZFReader;
use crate::output::Output;
use crate::misses::MissesFile;

pub fn run() -> Result<(), Error> {
    let config = config::get_config()?;
    let input = Input::from_file(&config.input_file)?;
    let bgzf = BGZFReader::new(File::open(config.data_file)?);
    let tabix =
        Tabix::from_reader(&mut File::open(&config.index_file)?)?;
    let output = match config.output_file_opt {
        None => { Output::from_stdout() }
        Some(output_file) => { Output::from_file(output_file)? }
    };
    let misses_file = match config.cache_misses_file_opt {
        None => { MissesFile::from_stdout()? }
        Some(cache_misses_file) => { MissesFile::from_file(cache_misses_file)? }
    };
    for name_raw in &tabix.names {
        let name = String::from_utf8(name_raw.clone())?;
        println!("Chromosome: {}", name)
    }
    join::join_input_with_data(input, bgzf, tabix, output, misses_file, config.i_col_ref,
                               config.i_col_alt)?;
    Ok(())
}