use crate::util::error::Error;
use clap::{App, SubCommand, Arg};

pub(crate) struct Config {
    pub(crate) data_file: String,
    pub(crate) index_file: String,
    pub(crate) input_file: String,
    pub(crate) cache_misses_file_opt: Option<String>,
    pub(crate) output_file_opt: Option<String>,
    pub(crate) i_col_ref: usize,
    pub(crate) i_col_alt: usize,
}

impl Config {
    fn new(data_file: String, index_file_opt: Option<String>, input_file: String,
           cache_misses_file_opt: Option<String>, output_file_opt: Option<String>,
           i_col_ref: usize, i_col_alt: usize)
           -> Config {
        let index_file =
            match index_file_opt {
                Some(index_file) => index_file,
                None => data_file.clone() + ".tbi"
            };
        Config {
            data_file,
            index_file,
            input_file,
            cache_misses_file_opt,
            output_file_opt,
            i_col_ref,
            i_col_alt,
        }
    }
}

mod names {
    pub(crate) const TABIX: &str = "tabix";
    pub(crate) const DATA_FILE: &str = "data-file";
    pub(crate) const INDEX_FILE: &str = "index-file";
    pub(crate) const INPUT_FILE: &str = "input-file";
    pub(crate) const CACHE_MISSES_FILE: &str = "cache-misses-file";
    pub(crate) const OUTPUT_FILE: &str = "output-file";
    pub(crate) const I_COL_REF: &str = "i-col-ref";
    pub(crate) const I_COL_ALT: &str = "i-col-alt";
}

pub(crate) fn get_config() -> Result<Config, Error> {
    let app =
        App::new(clap::crate_name!())
            .author(clap::crate_authors!())
            .version(clap::crate_version!())
            .about(clap::crate_description!())
            .subcommand(
                SubCommand::with_name(names::TABIX)
                    .help("Reads region from block-gzipped, tabix-index, location-sorted TSV file")
                    .arg(Arg::with_name(names::DATA_FILE)
                        .short("d")
                        .long("data-file")
                        .takes_value(true)
                        .required(true)
                        .help("The data file")
                    )
                    .arg(Arg::with_name(names::INDEX_FILE)
                        .short("t")
                        .long("index-file")
                        .takes_value(true)
                        .help("The index file")
                    )
                    .arg(Arg::with_name(names::INPUT_FILE)
                        .short("i")
                        .long("input-file")
                        .takes_value(true)
                        .required(true)
                        .help("The input file")
                    )
                    .arg(Arg::with_name(names::CACHE_MISSES_FILE)
                        .short("c")
                        .long("cache-misses-file")
                        .takes_value(true)
                        .help("The file to write cache misses.")
                    )
                    .arg(Arg::with_name(names::OUTPUT_FILE)
                        .short("o")
                        .long("output-file")
                        .takes_value(true)
                        .help("The output file")
                    )
                    .arg(Arg::with_name(names::I_COL_REF)
                        .short("r")
                        .long("i-col-ref")
                        .takes_value(true)
                        .help("The column in data file containing the ref allele")
                    )
                    .arg(Arg::with_name(names::I_COL_ALT)
                        .short("a")
                        .long("i-col-alt")
                        .takes_value(true)
                        .help("The column in data file containing the alt allele")
                    )
            );
    let matches = app.get_matches_safe()?;
    if let Some(tabix_matches) = matches.subcommand_matches(names::TABIX) {
        let data_file =
            String::from(tabix_matches.value_of(names::DATA_FILE)
                .ok_or_else(|| Error::from("Missing argument --data-file."))?);
        let index_file_opt =
            tabix_matches.value_of(names::INDEX_FILE).map(String::from);
        let input_file =
            String::from(tabix_matches.value_of(names::INPUT_FILE)
                .ok_or_else(|| Error::from("Missing argument --input-file."))?);
        let cache_misses_file_opt =
            tabix_matches.value_of(names::CACHE_MISSES_FILE).map(String::from);
        let output_file_opt =
            tabix_matches.value_of(names::OUTPUT_FILE).map(String::from);
        let i_col_ref =
            str::parse::<usize>(tabix_matches.value_of(names::I_COL_REF)
                .ok_or_else(|| Error::from("Missing argument --i-col-ref"))?)?;
        let i_col_alt =
            str::parse::<usize>(tabix_matches.value_of(names::I_COL_ALT)
                .ok_or_else(|| Error::from("Missing argument --i-col-alt"))?)?;
        Ok(Config::new(data_file, index_file_opt, input_file, cache_misses_file_opt,
                       output_file_opt, i_col_ref, i_col_alt))
    } else {
        Err(Error::from("Need to specify sub-command."))
    }
}