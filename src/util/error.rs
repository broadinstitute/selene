use std::fmt::{Display, Formatter, Debug};
use std::{fmt, io, string};
use clap::ErrorKind;
use std::string::FromUtf8Error;
use std::num::ParseIntError;
use bgzip::BGZFError;
use nom::error::VerboseError;
use nom::Err;
use std::any::Any;
use std::str::Utf8Error;

pub struct SeleneError {
    message: String,
}

pub enum Error {
    Selene(SeleneError),
    Clap(clap::Error),
    IO(io::Error),
    Utf8(string::FromUtf8Error),
    ParseInt(ParseIntError),
    Bgzf(BGZFError),
    Nom(String),
    Utf8str(Utf8Error),
}

pub(crate) trait Reporter {
    fn report(&self, error: Error);
}

pub(crate) fn handle_result<T>(result: Result<T, Error>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            println!("Error: {}", error);
            None
        }
    }
}

impl Error {
    pub fn is_clap_pseudo_error(&self) -> bool {
        if let Error::Clap(clap_error) = &self {
            matches!(clap_error.kind, ErrorKind::HelpDisplayed | ErrorKind::VersionDisplayed)
        } else {
            false
        }
    }
    pub fn error_code(&self) -> i32 {
        match self {
            Error::Selene(_) => { 1 }
            Error::Clap(_) => { 2 }
            Error::IO(_) => { 3 }
            Error::Utf8(_) => { 4 }
            Error::ParseInt(_) => { 5 }
            Error::Bgzf(_) => { 6 }
            Error::Nom(_) => { 7 }
            Error::Utf8str(_) => { 8 }
        }
    }
}

impl From<String> for SeleneError {
    fn from(message: String) -> Self { SeleneError { message } }
}

impl From<&str> for SeleneError {
    fn from(message_str: &str) -> Self { SeleneError::from(String::from(message_str)) }
}

impl From<SeleneError> for Error {
    fn from(selene_error: SeleneError) -> Self { Error::Selene(selene_error) }
}

impl From<String> for Error {
    fn from(message: String) -> Self { Error::Selene(SeleneError::from(message)) }
}

impl From<&str> for Error {
    fn from(message_str: &str) -> Self { Error::Selene(SeleneError::from(message_str)) }
}

impl From<clap::Error> for Error {
    fn from(clap_error: clap::Error) -> Self { Error::Clap(clap_error) }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self { Error::IO(io_error) }
}

impl From<string::FromUtf8Error> for Error {
    fn from(utf8error: FromUtf8Error) -> Self { Error::Utf8(utf8error) }
}

impl From<ParseIntError> for Error {
    fn from(parse_int_error: ParseIntError) -> Self { Error::ParseInt(parse_int_error) }
}

impl From<BGZFError> for Error {
    fn from(bgzf_error: BGZFError) -> Self { Error::Bgzf(bgzf_error) }
}

impl From<nom::Err<VerboseError<&str>>> for Error {
    fn from(nom_error: Err<VerboseError<&str>>) -> Self {
        Error::Nom(format!("{}", nom_error))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(utf8_error: Utf8Error) -> Self {
        Error::Utf8str(utf8_error)
    }
}

impl Display for SeleneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.message, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Selene(selene_error) => { selene_error.fmt(f) }
            Error::Clap(clap_error) => { std::fmt::Display::fmt(&clap_error, f) }
            Error::IO(io_error) => { fmt::Display::fmt(&io_error, f) }
            Error::Utf8(utf8error) => { fmt::Display::fmt(&utf8error, f) }
            Error::ParseInt(parse_int_error) => {
                fmt::Display::fmt(&parse_int_error, f)
            }
            Error::Bgzf(bgzf_error) => { fmt::Display::fmt(&bgzf_error, f) }
            Error::Nom(nom_error) => { fmt::Display::fmt(&nom_error, f) }
            Error::Utf8str(utf8_error) => { fmt::Display::fmt(utf8_error, f) }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl From<Box<dyn Any + Send>> for Error {
    fn from(join_error: Box<dyn Any + Send>) -> Self {
        let panic_message =
            if let Some(string) = join_error.downcast_ref::<String>() {
                string.as_str()
            } else if let Some(string) = join_error.downcast_ref::<&str>() {
                string
            } else {
                "(no more information)"
            };
        Error::from(format!("Joining panicked thread: {}", panic_message))
    }
}
