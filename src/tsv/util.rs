use crate::util::error::Error;
use std::collections::{HashSet, HashMap};
use std::fmt::{Display, Formatter};
use crate::util::iter_util;

pub(crate) fn col_indices_from_header_line(line: &str, cols: &[&str])
                                           -> Result<Vec<usize>, Error> {
    let mut cols_wanted: HashSet<&str> = HashSet::new();
    for col in cols {
        cols_wanted.insert(col);
    }
    let mut i_cols_map = HashMap::<&str, usize>::new();
    for (i, col_line) in line.split('\t').enumerate() {
        let mut cols_new = Vec::<&str>::new();
        for col_wanted in &cols_wanted {
            if *col_wanted == col_line {
                cols_new.push(col_line);
            }
        }
        for col_new in cols_new {
            cols_wanted.remove(col_new);
            i_cols_map.insert(col_new, i);
        }
        if cols_wanted.is_empty() {
            break;
        }
    }
    if cols_wanted.is_empty() {
        let i_cols: Vec<usize> =
            cols.iter().map(|col| { *i_cols_map.get(col).unwrap() }).collect();
        Ok(i_cols)
    } else {
        Err(Error::from(
            format!("Missing columns {}.", DisplayHashSet { hash_set: cols_wanted })
        ))
    }
}

pub(crate) fn extract_data_from_line<'a>(line: &'a str, i_cols: &[usize])
                                         -> Result<Vec<&'a str>, Error> {
    let parts: Vec<&'a str> = line.split('\t').collect();
    let mut values = Vec::<&'a str>::new();
    for i_col in i_cols {
        let part_i =
            parts.get(*i_col).ok_or_else(|| {
                Error::from(format!("Need field {}, but row has only {} fields.",
                                    i_col, parts.len()))
            })?;
        values.push(part_i)
    }
    Ok(values)
}

struct DisplayHashSet<'a> {
    hash_set: HashSet<&'a str>,
}

impl Display for DisplayHashSet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        iter_util::fmt_set("", &self.hash_set, "", f)
    }
}