use std::convert::Infallible;
use std::{error::Error, fmt};
use std::io::Error as IoError;
use rusqlite::Error as sqlError;
#[derive(Debug)]
pub struct ApplicationError {
    details: String
}

impl ApplicationError {
    fn new(msg: String) -> ApplicationError {
        ApplicationError{details: msg}
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}


impl Error for ApplicationError {
    fn description(&self) -> &str {
        &self.details
    }
    
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl From<IoError> for ApplicationError{
    fn from(value: IoError) -> Self {
        ApplicationError::new(value.to_string())
    }
}

impl  From<sqlError> for ApplicationError{
    fn from(value: sqlError) -> Self {
        ApplicationError::new(value.to_string())
    }
}

impl From<eframe::Error> for ApplicationError {
    fn from(value: eframe::Error) -> Self {
        ApplicationError::new(value.to_string())
    }
}

impl  From<String> for ApplicationError{
    fn from(value: String) -> Self {
        ApplicationError::new(value)
    }
}


impl  From<Infallible> for ApplicationError{
    fn from(value: Infallible) -> Self {
        ApplicationError::new(value.to_string())
    }
}

impl  From<refinery::Error> for ApplicationError{
    fn from(value: refinery::Error) -> Self {
        ApplicationError::new(value.to_string())
    }
}