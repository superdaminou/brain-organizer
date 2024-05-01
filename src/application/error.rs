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

impl Default for ApplicationError {
    fn default() -> Self {
        Self { details: String::from("An error occured") }
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

impl From<indradb::Error> for ApplicationError {
    fn from(value: indradb::Error) -> Self {
        ApplicationError::new(value.to_string())
    }
}

impl From<indradb::ValidationError> for ApplicationError {
    fn from(value: indradb::ValidationError) -> Self {
        ApplicationError::new(value.to_string())
    }
}

impl  From<String> for ApplicationError{
    fn from(value: String) -> Self {
        ApplicationError::new(value)
    }
}

impl  From<&str> for ApplicationError{
    fn from(value: &str) -> Self {
        ApplicationError::new(value.to_string())
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

impl  From<uuid::Error> for ApplicationError{
    fn from(value: uuid::Error) -> Self {
        ApplicationError::new(value.to_string())
    }
}