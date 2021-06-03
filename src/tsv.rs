use std::io::BufRead;
use crate::util::error::Error;

pub(crate) struct IAlleleCols {
    pub(crate) i_col_ref: usize,
    pub(crate) i_col_alt: usize,
}

impl IAlleleCols {
    pub(crate) fn new(i_col_ref: usize, i_col_alt: usize) -> IAlleleCols {
        IAlleleCols { i_col_ref, i_col_alt }
    }
    pub(crate) fn parse(header_line: &str, col_ref: &str, col_alt: &str)
                        -> Result<IAlleleCols, Error> {
        let stripped_line =
            header_line.strip_prefix('#').unwrap_or(&header_line);
        let mut i_col_ref_opt: Option<usize> = None;
        let mut i_col_alt_opt: Option<usize> = None;
        let mut parts = stripped_line.split('\t');
        let mut count: usize = 0;
        let mut is_exhausted: bool = false;
        while !is_exhausted && (i_col_ref_opt.is_none() || i_col_alt_opt.is_none()) {
            match parts.next() {
                None => { is_exhausted = true }
                Some(field) => {
                    if field == col_ref { i_col_ref_opt = Some(count) }
                    if field == col_alt { i_col_alt_opt = Some(count) }
                }
            }
            count += 1;
        };
        match (i_col_ref_opt, i_col_alt_opt) {
            (Some(i_col_ref), Some(i_col_alt)) => Ok(IAlleleCols::new(i_col_ref, i_col_alt)),
            (None, _) => {
                Err(Error::from(format!("No field {} in header line", col_ref)))
            }
            (_, None) => {
                Err(Error::from(format!("No field {} in header line", col_alt)))
            }
        }
    }
}

pub(crate) fn get_header_line<R: BufRead>(read: &mut R) -> Result<String, Error> {
    let res_opt = read.lines().find(|res| {
        match res {
            Ok(string) => { !string.starts_with("##") }
            Err(_) => false
        }
    });
    match res_opt {
        None => { Err(Error::from("No header line found.")) }
        Some(Err(error)) => { Err(Error::from(error)) }
        Some(Ok(line)) => Ok(line)
    }
}



