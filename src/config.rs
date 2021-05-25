use crate::util::error::Error;
use clap::{App, SubCommand, Arg};

pub(crate) struct Config {
    pub(crate) data_file: String,
    pub(crate) index_file: String,
    pub(crate) input_file: String,
    pub(crate) output_file_opt: Option<String>,
}

impl Config {
    fn new(data_file: String, index_file_opt: Option<String>, input_file: String,
                  output_file_opt: Option<String>) -> Config {
        let index_file =
            match index_file_opt {
            Some(index_file) => index_file,
                None => data_file.clone() + ".tbi"
        };
        Config { data_file, index_file, input_file, output_file_opt }
    }
}

mod names {
    pub(crate) const TABIX: &str = "tabix";
    pub(crate) const DATA_FILE: &str = "data-file";
    pub(crate) const INDEX_FILE: &str = "index-file";
    pub(crate) const INPUT_FILE: &str = "input-file";
    pub(crate) const OUTPUT_FILE: &str = "output-file";
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
                    .arg(Arg::with_name(names::OUTPUT_FILE)
                        .short("o")
                        .long("output-file")
                        .takes_value(true)
                        .help("The output file")
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
        let output_file_opt =
            tabix_matches.value_of(names::OUTPUT_FILE).map(String::from);
        Ok(Config::new(data_file, index_file_opt, input_file, output_file_opt))
    } else {
        Err(Error::from("Need to specify sub-command."))
    }
}