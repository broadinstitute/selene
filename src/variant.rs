use crate::util::error::Error;
use std::str::Split;

pub(crate) struct Variant {
    chrom: String,
    pos: u32,
    ref_allele: String,
    alt_allele: String,
}

impl Variant {
    pub(crate) fn new(chrom: String, pos: u32, ref_allele: String, alt_allele: String) -> Variant {
        Variant { chrom, pos, ref_allele, alt_allele }
    }
    pub(crate) fn canonical_id(&self) -> String {
        format!("{}:{}_{}/{}", self.chrom, self.pos, self.ref_allele, self.alt_allele)
    }
    pub(crate) fn end(&self) -> u32 { self.pos + (self.ref_allele.len() as u32) }
}

pub(crate) fn parse_line(line: String) -> Result<Variant, Error> {
    let mut fields = line.split('\t');
    let chrom = String::from(get_field(&mut fields, "CHROM")?);
    let pos = str::parse::<u32>(get_field(&mut fields, "POS")?)?;
    get_field(&mut fields, "ID")?;
    let ref_allele = String::from(get_field(&mut fields, "REF")?);
    let alt_allele = String::from(get_field(&mut fields, "ALT")?);
    Ok(Variant::new(chrom, pos, ref_allele, alt_allele))
}

fn get_field<'a, 'b>(fields: &'a mut Split<'b, char>, field_name: &str) -> Result<&'b str, Error> {
    fields.next().ok_or(Error::from(format!("Missing {} field", field_name)))
}

