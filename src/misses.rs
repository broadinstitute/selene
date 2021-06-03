use std::io::{Write, LineWriter};
use crate::util::error::Error;
use std::fs::File;
use crate::variant::Variant;

pub(crate) struct MissesFile {
    write: LineWriter<Box<dyn Write>>,
}

impl MissesFile {
    fn write_header(write: &mut LineWriter<Box<dyn Write>>) -> Result<(), Error> {
        let header = "#CHROM\tPOS\tID\tREF\tALT\n";
        write.write_all(header.as_bytes())?;
        Ok(())
    }
    pub(crate) fn from_file(out_file: String) -> Result<MissesFile, Error> {
        let mut write: LineWriter<Box<dyn Write>> =
            LineWriter::new(Box::new(File::create(out_file)?));
        MissesFile::write_header(&mut write)?;
        Ok(MissesFile { write })
    }
    pub(crate) fn from_stdout() -> Result<MissesFile, Error> {
        let mut write: LineWriter<Box<dyn Write>> =
            LineWriter::new(Box::new(std::io::stdout()));
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