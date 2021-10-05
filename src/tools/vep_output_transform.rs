use crate::util::error::Error;
use std::io::{BufReader, BufWriter, BufRead, Write, Read};
use fs_err::File;
use std::{cmp, io};

const PICK_COL: &str = "PICK";
const PICK_VALUE: &str = "1";
const LOCATION_COL: &str = "Location";
const CHROM_COL: &str = "Chrom";
const POS_COL: &str = "Pos";

pub(crate) fn transform_vep_output_files(input_file: &str, output_file: &str)
                                         -> Result<(), Error> {
    let reader = BufReader::new(File::open(input_file)?);
    let writer = BufWriter::new(File::create(output_file)?);
    transform_vep_output(reader, writer)
}

pub(crate) fn transform_vep_output_pipe() -> Result<(), Error> {
    let reader = BufReader::new(io::stdin());
    let writer = BufWriter::new(io::stdout());
    transform_vep_output(reader, writer)
}

pub(crate) fn transform_vep_output<I, O>(reader: BufReader<I>, mut writer: BufWriter<O>)
                                         -> Result<(), Error>
    where I: Read, O: Write {
    const I_DUMMY: usize = 0;
    let mut i_pick: usize = I_DUMMY;
    let mut i_location: usize = I_DUMMY;
    for line_res in reader.lines() {
        let line = line_res?;
        if line.starts_with("##") {
            writeln!(&mut writer, "{}", &line)?;
        } else if let Some(line_stripped) = line.strip_prefix('#') {
            let mut i_pick_opt: Option<usize> = None;
            let mut i_location_opt: Option<usize> = None;
            for (i, column) in line_stripped.split('\t').enumerate() {
                if column == PICK_COL {
                    i_pick_opt = Some(i);
                } else if column == LOCATION_COL {
                    i_location_opt = Some(i)
                }
                if i_pick_opt.is_some() && i_location_opt.is_some() {
                    break;
                }
            }
            i_pick = i_pick_opt.ok_or_else(|| { format!("Missing column {}", PICK_COL) })?;
            i_location =
                i_location_opt.ok_or_else(|| { format!("Missing column {}", LOCATION_COL) })?;
            writeln!(&mut writer, "#{}\t{}\t{}", CHROM_COL, POS_COL, &line_stripped)?;
        } else {
            let mut pick_opt: Option<&str> = None;
            let mut location_opt: Option<&str> = None;
            let i_limit = cmp::max(i_pick, i_location) + 1;
            let mut value_iter = line.split('\t');
            for i in 0..i_limit {
                match value_iter.next() {
                    None => {
                        return Err(Error::from(
                            format!("Expected {} columns, but only got {}.", i_limit, i)
                        ));
                    }
                    Some(value) => {
                        if i == i_pick {
                            pick_opt = Some(value)
                        } else if i == i_location {
                            location_opt = Some(value)
                        }
                    }
                }
            }
            let pick = pick_opt.unwrap();
            if pick == PICK_VALUE {
                let location = location_opt.unwrap();
                let mut location_parts_iter = location.split(':');
                let chrom = location_parts_iter.next().ok_or_else(|| {
                    Error::from(format!("Cannot parse location {}.", location))
                })?;
                let position = location_parts_iter.next().ok_or_else(|| {
                    Error::from(format!("Cannot parse location {}.", location))
                })?.split('-').next().ok_or_else(|| {
                    Error::from(format!("Cannot parse location {}.", location))
                })?;
                writeln!(&mut writer, "{}\t{}\t{}", chrom, position, &line)?;
            }
        }
    }
    Ok(())
}


