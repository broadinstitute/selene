use std::io::{Write, BufWriter};
use crate::util::error::Error;
use fs_err::File;
use crate::genomics::variant::Variant;
use crate::cache::meta_lines;

pub(crate) struct MissesFile {
    write: BufWriter<Box<dyn Write>>,
}

impl MissesFile {
    fn write_header(write: &mut BufWriter<Box<dyn Write>>) -> Result<(), Error> {
        let header = "#CHROM\tPOS\tID\tREF\tALT\n";
        write.write_all(header.as_bytes())?;
        Ok(())
    }
    pub(crate) fn from_file(out_file: String, meta_lines: &[String]) -> Result<MissesFile, Error> {
        let mut write: BufWriter<Box<dyn Write>> =
            BufWriter::new(Box::new(File::create(out_file)?));
        meta_lines::write_meta_lines(&mut write, meta_lines)?;
        MissesFile::write_header(&mut write)?;
        Ok(MissesFile { write })
    }
    pub(crate) fn from_stdout(meta_lines: &[String]) -> Result<MissesFile, Error> {
        let mut write: BufWriter<Box<dyn Write>> =
            BufWriter::new(Box::new(std::io::stdout()));
        meta_lines::write_meta_lines(&mut write, meta_lines)?;
        MissesFile::write_header(&mut write)?;
        Ok(MissesFile { write })
    }
    pub(crate) fn write_variant(&mut self, variant: &Variant) -> Result<(), Error>{
        let line =
            format!("{}\t{}\t{}\t{}\t{}\n", variant.chrom, variant.pos, variant.canonical_id(),
                    variant.ref_allele, variant.alt_allele);
        self.write.write_all(line.as_bytes())?;
        Ok(())
    }
}