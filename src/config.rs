use crate::util::error::Error;
use clap::{App, SubCommand, Arg};

pub(crate) struct Config {
    pub(crate) data_file: String,
    pub(crate) index_file: String,
}

impl Config {
    fn from_data_file(data_file: String) -> Config {
        let index_file = data_file.clone() + ".tbi";
        Config { data_file, index_file }
    }
    fn from_data_and_index_file(data_file: String, index_file: String) -> Config {
        Config { data_file, index_file }
    }
    fn from_data_and_index_file_opt(data_file: String, index_file_opt: Option<String>) -> Config {
        match index_file_opt {
            None => { Config::from_data_file(data_file) }
            Some(index_file) => { Config::from_data_and_index_file(data_file, index_file) }
        }
    }
}

mod names {
    pub(crate) const TABIX: &str = "tabix";
    pub(crate) const DATA_FILE: &str = "data-file";
    pub(crate) const INDEX_FILE: &str = "index-file";
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
                        .short("i")
                        .long("index-file")
                        .takes_value(true)
                        .help("The index file")
                    )
            );
    let matches = app.get_matches_safe()?;
    if let Some(tabix_matches) = matches.subcommand_matches(names::TABIX) {
        let data_file =
            String::from(tabix_matches.value_of(names::DATA_FILE)
                .ok_or_else(|| Error::from("Missing argument --data-file."))?);
        let index_file_opt =
            tabix_matches.value_of(names::INDEX_FILE).map(String::from);
        Ok(Config::from_data_and_index_file_opt(data_file, index_file_opt))
    } else {
        Err(Error::from("Need to specify sub-command."))
    }
}