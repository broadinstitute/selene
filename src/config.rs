use crate::util::error::Error;
use clap::{App, SubCommand, Arg};

pub(crate) enum Config {
    Tabix(TabixConfig),
    Script(ScriptConfig),
}

pub(crate) struct TabixConfig {
    pub(crate) input_config: TabixInputConfig,
    pub(crate) cache_misses_file_opt: Option<String>,
    pub(crate) output_file_opt: Option<String>,
}

pub(crate) struct TabixInputConfig {
    pub(crate) cache_file: String,
    pub(crate) index_file: String,
    pub(crate) input_file: String,
    pub(crate) regions_file_opt: Option<String>,
    pub(crate) col_ref: String,
    pub(crate) col_alt: String,
}

pub(crate) struct ScriptConfig {
    pub(crate) script_file: String,
}

impl TabixConfig {
    pub(crate) fn new(input_config: TabixInputConfig, cache_misses_file_opt: Option<String>,
           output_file_opt: Option<String>)
           -> TabixConfig {
        TabixConfig {
            input_config,
            cache_misses_file_opt,
            output_file_opt,
        }
    }
}

impl TabixInputConfig {
    pub(crate) fn new(cache_file: String, index_file_opt: Option<String>, input_file: String,
                      regions_file_opt: Option<String>, col_ref: String, col_alt: String)
                      -> TabixInputConfig {
        let index_file =
            match index_file_opt {
                Some(index_file) => index_file,
                None => cache_file.clone() + ".tbi"
            };
        TabixInputConfig {
            cache_file,
            index_file,
            input_file,
            regions_file_opt,
            col_ref,
            col_alt,
        }
    }
}

impl ScriptConfig {
    fn new(script_file: String) -> ScriptConfig {
        ScriptConfig { script_file }
    }
}

mod names {
    pub(crate) const TABIX: &str = "tabix";
    pub(crate) const SCRIPT: &str = "script";
    pub(crate) const DATA_FILE: &str = "data-file";
    pub(crate) const INDEX_FILE: &str = "index-file";
    pub(crate) const INPUT_FILE: &str = "input-file";
    pub(crate) const REGIONS_FILE: &str = "regions-file";
    pub(crate) const CACHE_MISSES_FILE: &str = "cache-misses-file";
    pub(crate) const OUTPUT_FILE: &str = "output-file";
    pub(crate) const COL_REF: &str = "col-ref";
    pub(crate) const COL_ALT: &str = "col-alt";
    pub(crate) const SCRIPT_FILE: &str = "script-file";
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
                    .arg(Arg::with_name(names::REGIONS_FILE)
                        .short("g")
                        .long("regions-file")
                        .takes_value(true)
                        .required(false)
                        .help("Optional file with regions. If provided, only variants within regions will be considered.")
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
                    .arg(Arg::with_name(names::COL_REF)
                        .short("r")
                        .long("col-ref")
                        .takes_value(true)
                        .help("The column name in data file containing the ref allele")
                    )
                    .arg(Arg::with_name(names::COL_ALT)
                        .short("a")
                        .long("col-alt")
                        .takes_value(true)
                        .help("The column name in data file containing the alt allele")
                    )
            )
            .subcommand(
                SubCommand::with_name(names::SCRIPT)
                    .arg(Arg::with_name(names::SCRIPT_FILE)
                        .value_name("script file")
                        .takes_value(true))
            );
    let matches = app.get_matches();
    if let Some(tabix_matches) = matches.subcommand_matches(names::TABIX) {
        let data_file =
            String::from(tabix_matches.value_of(names::DATA_FILE)
                .ok_or_else(|| Error::from("Missing argument --data-file."))?);
        let index_file_opt =
            tabix_matches.value_of(names::INDEX_FILE).map(String::from);
        let input_file =
            String::from(tabix_matches.value_of(names::INPUT_FILE)
                .ok_or_else(|| Error::from("Missing argument --input-file."))?);
        let regions_file_opt =
            tabix_matches.value_of(names::REGIONS_FILE).map(String::from);
        let cache_misses_file_opt =
            tabix_matches.value_of(names::CACHE_MISSES_FILE).map(String::from);
        let output_file_opt =
            tabix_matches.value_of(names::OUTPUT_FILE).map(String::from);
        let col_ref =
            String::from(tabix_matches.value_of(names::COL_REF)
                .ok_or_else(|| Error::from("Missing argument --col-ref."))?);
        let col_alt =
            String::from(tabix_matches.value_of(names::COL_ALT)
                .ok_or_else(|| Error::from("Missing argument --col-alt."))?);
        let input_config =
            TabixInputConfig::new(data_file, index_file_opt, input_file, regions_file_opt, col_ref,
                                  col_alt);
        let tabix_config =
            TabixConfig::new(input_config, cache_misses_file_opt, output_file_opt);
        Ok(Config::Tabix(tabix_config))
    } else if let Some(script_matches) =
    matches.subcommand_matches(names::SCRIPT) {
        let script_file =
            String::from(script_matches.value_of(names::SCRIPT_FILE)
                .ok_or_else(|| Error::from("Missing argument for script file."))?);
        let script_config = ScriptConfig::new(script_file);
        Ok(Config::Script(script_config))
    } else {
        Err(Error::from(format!("Need to specify sub-command ({} or {}).",
                                names::TABIX, names::SCRIPT)))
    }
}