use std::io::{Read, BufReader, BufRead};
use crate::variant::Variant;
use crate::util::error::Error;
use std::fs::File;
use crate::variant;
use crate::util::error;

pub(crate) struct Input {
    reader: BufReader<Box<dyn Read>>
}

impl Input {
    pub(crate) fn from_file(file: &str) -> Result<Input, Error> {
        let inner: Box<dyn Read> = Box::new(File::open(file)?);
        let reader = BufReader::new(inner);
        Ok(Input { reader })
    }
    pub(crate) fn variants<'a>(self) -> impl Iterator<Item=(Variant, String)> + 'a {
        self.reader.lines().filter_map(|line_res|{
             match line_res {
                Ok(line) => Some(line),
                Err(io_error) => {
                    println!("Error: {}", Error::IO(io_error));
                    None
                }
            }
        })
            .filter(|line| { ! line.starts_with('#')})
            .map(variant::parse_vcf_line)
            .filter_map(error::handle_result)
    }
}

