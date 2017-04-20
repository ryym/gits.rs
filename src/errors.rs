use std::io;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref e) => e.fmt(f),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CliError::Io(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}
