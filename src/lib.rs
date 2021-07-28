mod util;
mod config;
mod input;
mod variant;
mod output;
mod misses;
mod join;
mod tabix;
mod tsv;
mod meta_lines;
mod regions;
mod mion;

#[macro_use]
extern crate nom;

use crate::util::error::Error;
use std::fs::File;
use bgzip::tabix::Tabix;
use crate::input::Input;
use bgzip::BGZFReader;
use crate::output::Output;
use crate::misses::MissesFile;
use crate::tsv::IAlleleCols;
use crate::regions::Regions;

pub fn run() -> Result<(), Error> {
    let config = config::get_config()?;
    let input_config = &config.input_config;
    let input = Input::from_file(&input_config.input_file)?;
    let mut bgzf =
        BGZFReader::new(File::open(&input_config.data_file)?);
    let tabix =
        Tabix::from_reader(&mut File::open(&input_config.index_file)?)?;
    let chroms: Vec<String> = tabix.names.iter().filter_map(|raw| {
       String::from_utf8(raw.clone()).ok()
    }).collect();
    let chroms_line = meta_lines::chromosome_line(chroms.as_slice());
    let meta_lines = vec!(chroms_line);
    let header_line = tsv::get_header_line(&mut bgzf)?;
    let regions_opt = match &input_config.regions_file_opt {
        None => { None }
        Some(regions_file) => { Some(Regions::load(regions_file)?) }
    };
    let output = match config.output_file_opt {
        None => { Output::from_stdout(&header_line, &meta_lines)? }
        Some(output_file) => {
            Output::from_file(output_file, &header_line, &meta_lines)?
        }
    };
    let misses_file = match config.cache_misses_file_opt {
        None => { MissesFile::from_stdout(&meta_lines)? }
        Some(cache_misses_file) => {
            MissesFile::from_file(cache_misses_file, &meta_lines)?
        }
    };
    for name_raw in &tabix.names {
        let name = String::from_utf8(name_raw.clone())?;
        println!("Chromosome: {}", name)
    }
    let i_allele_cols =
        IAlleleCols::parse(&header_line, &input_config.col_ref, &input_config.col_alt)?;
    join::join_input_with_data(input, bgzf, tabix, regions_opt, output, misses_file,
                               i_allele_cols)?;
    Ok(())
}