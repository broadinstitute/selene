use crate::util::error::Error;
use bgzip::BGZFReader;
use std::io::{Read, Seek};
use crate::input::Input;
use bgzip::tabix::{Tabix, TabixSequence};
use crate::output::Output;
use crate::misses::MissesFile;

pub(crate) fn join_input_with_data<R>(input: Input, bgzf: BGZFReader<R>, tabix: Tabix,
                                      output: Output, misses_file: MissesFile)
                                      -> Result<(), Error>
    where R: Read + Seek {
    let dummy_str = "";
    let mut sequence_name = String::from(dummy_str);
    let mut i_sequence_opt: Option<usize> = None;
    let mut sequence_opt: Option<&TabixSequence> = None;
    for variant in input.variants() {
        if variant.chrom != sequence_name {
            i_sequence_opt =
                tabix.names.iter()
                    .position(|name| sequence_name.as_bytes() == name);

        }

    }
    Ok(())
}