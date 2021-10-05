use crate::util::error::Error;
use std::io::{BufReader, BufWriter, BufRead, Write, Read};
use fs_err::File;
use std::{cmp, io};
use crate::genomics::variant::Variant;

const PICK_COL: &str = "PICK";
const PICK_VALUE: &str = "1";
const ID_COL: &str = "Uploaded_variation";
const CHROM_COL: &str = "Chrom";
const POS_COL: &str = "Pos";
const REF_COL: &str = "Ref";
const ALT_COL: &str = "Alt";

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
    let mut i_id: usize = I_DUMMY;
    for line_res in reader.lines() {
        let line = line_res?;
        if line.starts_with("##") {
            writeln!(&mut writer, "{}", &line)?;
        } else if let Some(line_stripped) = line.strip_prefix('#') {
            let mut i_pick_opt: Option<usize> = None;
            let mut i_id_opt: Option<usize> = None;
            for (i, column) in line_stripped.split('\t').enumerate() {
                if column == PICK_COL {
                    i_pick_opt = Some(i);
                } else if column == ID_COL {
                    i_id_opt = Some(i)
                }
                if i_pick_opt.is_some() && i_id_opt.is_some() {
                    break;
                }
            }
            i_pick = i_pick_opt.ok_or_else(|| { format!("Missing column {}", PICK_COL) })?;
            i_id =
                i_id_opt.ok_or_else(|| { format!("Missing column {}", ID_COL) })?;
            writeln!(&mut writer, "#{}\t{}\t{}\t{}\t{}", CHROM_COL, POS_COL, REF_COL, ALT_COL,
                     &line_stripped)?;
        } else {
            let mut pick_opt: Option<&str> = None;
            let mut id_opt: Option<&str> = None;
            let i_limit = cmp::max(i_pick, i_id) + 1;
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
                        } else if i == i_id {
                            id_opt = Some(value)
                        }
                    }
                }
            }
            let pick = pick_opt.unwrap();
            if pick == PICK_VALUE {
                let id = id_opt.unwrap();
                let variant = Variant::parse(id)?;
                writeln!(&mut writer, "{}\t{}\t{}\t{}\t{}", &variant.chrom, &variant.pos,
                         &variant.ref_allele, &variant.alt_allele, &line)?;
            }
        }
    }
    Ok(())
}


