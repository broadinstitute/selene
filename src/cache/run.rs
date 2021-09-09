use fs_err::File;

use bgzip::BGZFReader;
use bgzip::tabix::Tabix;

use crate::cache::{join, meta_lines};
use crate::cache::input::Input;
use crate::cache::misses::MissesFile;
use crate::cache::output::Output;
use crate::cache::regions::Regions;
use crate::config::TabixConfig;
use crate::tabix::tsv;
use crate::tabix::tsv::IAlleleCols;
use crate::util::error::Error;

pub(crate) fn run_cache(tabix_config: TabixConfig) -> Result<(), Error> {
    let input_config = &tabix_config.input_config;
    let input = Input::from_file(&input_config.input_file)?;
    let mut bgzf =
        BGZFReader::new(File::open(&input_config.cache_file)?);
    let tabix =
        Tabix::from_reader(&mut File::open(&input_config.index_file)?)?;
    let chroms: Vec<String> = tabix.names.iter().filter_map(|raw| {
        String::from_utf8(raw[0..(raw.len() - 1)].to_owned()).ok()
    }).collect();
    let vcf_version_line = String::from("##fileformat=VCFv4.0");
    let mut meta_lines = vec!(vcf_version_line);
    meta_lines.append(&mut meta_lines::chromosome_lines(chroms.as_slice()));
    let header_line = tsv::get_header_line(&mut bgzf)?;
    let regions_opt = match &input_config.regions_file_opt {
        None => { None }
        Some(regions_file) => { Some(Regions::load(regions_file)?) }
    };
    let output = match tabix_config.output_file_opt {
        None => { Output::from_stdout(&header_line, &meta_lines)? }
        Some(output_file) => {
            Output::from_file(output_file, &header_line, &meta_lines)?
        }
    };
    let misses_file = match tabix_config.cache_misses_file_opt {
        None => { MissesFile::from_stdout(&meta_lines)? }
        Some(cache_misses_file) => {
            MissesFile::from_file(cache_misses_file, &meta_lines)?
        }
    };
    let i_allele_cols =
        IAlleleCols::parse(&header_line, &input_config.col_ref, &input_config.col_alt)?;
    join::join_input_with_data(input, bgzf, tabix, regions_opt, output, misses_file,
                               i_allele_cols)?;
    Ok(())
}
