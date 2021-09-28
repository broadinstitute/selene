use crate::util::error::Error;

pub(crate) enum Hg { Hg19, Hg38 }

const HG19: &str = "hg19";
const HG38: &str = "hg38";
const GRCH37: &str = "GRCh37";
const GRCH38: &str = "GRCh38";

impl Hg {
    pub(crate) fn as_grc_str(&self) -> &str {
        match self {
            Hg::Hg19 => { GRCH37 }
            Hg::Hg38 => { GRCH38 }
        }
    }
    pub(crate) fn parse(string: &str) -> Result<Hg, Error> {
        match string {
            HG19 | GRCH37 => Ok(Hg::Hg19),
            HG38 | GRCH38 => Ok(Hg::Hg38),
            _ => Err(Error::from(format!("Unknown assembly '{}'.", string)))
        }
    }
}