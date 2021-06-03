use crate::util::error::Error;
use std::str::Split;
use std::cmp;

pub(crate) type Pos = u32;

pub(crate) struct ICols {
    pub(crate) i_col_chrom: usize,
    pub(crate) i_col_pos: usize,
    pub(crate) i_col_ref: usize,
    pub(crate) i_col_alt: usize,
}

#[derive(Eq, PartialEq)]
pub(crate) struct Variant {
    pub(crate) chrom: String,
    pub(crate) pos: u32,
    pub(crate) ref_allele: String,
    pub(crate) alt_allele: String,
}

impl ICols {
    pub(crate) fn new(i_col_chrom: usize, i_col_pos: usize, i_col_ref: usize, i_col_alt: usize)
                      -> ICols { ICols { i_col_chrom, i_col_pos, i_col_ref, i_col_alt } }
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

pub(crate) fn parse_vcf_line(line: String) -> Result<(Variant, String), Error> {
    let mut fields = line.split('\t');
    let chrom = String::from(get_vcf_field(&mut fields, "CHROM")?);
    let pos = str::parse::<u32>(get_vcf_field(&mut fields, "POS")?)?;
    get_vcf_field(&mut fields, "ID")?;
    let ref_allele = String::from(get_vcf_field(&mut fields, "REF")?);
    let alt_allele = String::from(get_vcf_field(&mut fields, "ALT")?);
    Ok((Variant::new(chrom, pos, ref_allele, alt_allele), line))
}

fn get_vcf_field<'a, 'b>(fields: &'a mut Split<'b, char>, field_name: &str)
                         -> Result<&'b str, Error> {
    fields.next().ok_or_else(|| Error::from(format!("Missing {} field", field_name)))
}

pub(crate) fn parse_tsv_line(line: String, i_cols: &ICols) -> Result<(Variant, String), Error> {
    let i_max =
        cmp::max(cmp::max(i_cols.i_col_chrom, i_cols.i_col_pos),
                 cmp::max(i_cols.i_col_ref, i_cols.i_col_alt)) + 1;
    let fields: Vec<&str> = line.split('\t').take(i_max).collect();
    let chrom = String::from(get_tsv_field(&fields, i_cols.i_col_chrom)?);
    let pos = str::parse::<u32>(get_tsv_field(&fields, i_cols.i_col_pos)?)?;
    let ref_allele = String::from(get_tsv_field(&fields, i_cols.i_col_ref)?);
    let alt_allele = String::from(get_tsv_field(&fields, i_cols.i_col_alt)?);
    Ok((Variant::new(chrom, pos, ref_allele, alt_allele), line))
}

fn get_tsv_field<'b>(fields: &[&'b str], i_col: usize) -> Result<&'b str, Error> {
    let field =
        *fields.get(i_col)
            .ok_or_else(|| Error::from(format!("Missing field {}.", i_col)))?;
    Ok(field)
}

