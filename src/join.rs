use crate::util::error::Error;
use bgzip::BGZFReader;
use std::io::{Read, Seek, BufRead};
use crate::input::Input;
use bgzip::tabix::Tabix;
use crate::output::Output;
use crate::misses::MissesFile;
use crate::variant::ICols;
use crate::variant;
use crate::tabix;
use crate::tsv::IAlleleCols;

struct SequenceMeta {
    name: String,
    i_opt: Option<usize>,
}


impl SequenceMeta {
    fn new() -> SequenceMeta {
        SequenceMeta { name: String::from(""), i_opt: None }
    }
    fn update_from(&mut self, name: &str, names: &[Vec<u8>]) {
        if self.name != name {
            self.name = name.to_string();
            self.i_opt = names.iter().position(|name_i_bytes| {
                let name_i_raw = String::from_utf8(name_i_bytes.clone())
                    .unwrap_or_else(|_| String::from(""));
                let name_i = name_i_raw.trim_matches(char::from(0));
                name == name_i
            });
        }
    }
}

pub(crate) fn join_input_with_data<R>(input: Input, mut bgzf: BGZFReader<R>, tabix: Tabix,
                                      mut output: Output, mut misses_file: MissesFile,
                                      i_allele_cols: IAlleleCols)
                                      -> Result<(), Error>
    where R: Read + Seek {
    let mut meta = SequenceMeta::new();
    let i_cols =
        ICols::new((tabix.column_for_sequence - 1) as usize,
                   (tabix.column_for_begin - 1) as usize,
                   i_allele_cols.i_col_ref, i_allele_cols.i_col_alt);
    for (variant, _) in input.variants() {
        meta.update_from(&variant.chrom, &tabix.names);
        match meta.i_opt {
            None => {
                misses_file.write_variant(&variant)?;
            }
            Some(i_seq) => {
                let sequence =
                    tabix.sequences.get(i_seq).ok_or_else(|| {
                        Error::from(format!("Index {} out of range for sequences.", i_seq))
                    })?;
                let i_interval = tabix::pos_to_i_interval(variant.pos);
                match sequence.intervals.get(i_interval as usize) {
                    None => { misses_file.write_variant(&variant)?; }
                    Some(vpos_interval) => {
                        let bins = tabix::variant_to_bins(&variant);
                        let mut vposes: Vec<u64> =
                            bins.iter().flat_map(|k| sequence.bins.get(k))
                                .flat_map(|tabix_bin| { &tabix_bin.chunks })
                                .filter_map(|chunk| {
                                    if chunk.end <= *vpos_interval {
                                        None
                                    } else if chunk.begin <= *vpos_interval {
                                        Some(*vpos_interval)
                                    } else {
                                        Some(chunk.begin)
                                    }
                                }).collect();
                        vposes.sort_unstable();
                        vposes.dedup();
                        let mut found_variant = false;
                        for vpos in vposes {
                            bgzf.bgzf_seek(vpos)?;
                            loop {
                                let mut line_buf = String::new();
                                let n_bytes_read = bgzf.read_line(&mut line_buf)?;
                                if n_bytes_read == 0 {
                                    break;
                                }
                                let (data_variant, line) =
                                    variant::parse_tsv_line(line_buf, &i_cols)?;
                                if variant == data_variant {
                                    found_variant = true;
                                    output.write_line(line)?;
                                } else if (variant.chrom != data_variant.chrom) ||
                                    (variant.pos < data_variant.pos) {
                                    break;
                                }
                            }
                        }
                        if !found_variant {
                            misses_file.write_variant(&variant)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}