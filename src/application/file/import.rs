use std::fs::read_to_string;

use log::info;

use crate::application::{error::ApplicationError, file::lib::{copy_recursively, REFERENCE_FILE, REFLEXION_FILE, REFLEXION_STORAGE}, reference::{self, structs::reference::{CsvLine, Reference}}, reflexion::{self, structs::Reflexion}};

const IMPORT_STORAGE: &str = "./import/";


pub fn import() -> Result<(), ApplicationError> {
    info!("Start importing reference file: {}", REFERENCE_FILE);
    read_to_string(IMPORT_STORAGE.to_string() + REFERENCE_FILE) 
        .map_err(ApplicationError::from)?
        .lines()  // split the string into an iterator of string slices
        .map(Reference::try_from)
        .collect::<Result<Vec<Reference>, ApplicationError>>()?
        .iter()
        .try_for_each(reference::service::create)?;

    info!("Start importing reflexion file: {}", REFLEXION_FILE);
    read_to_string(IMPORT_STORAGE.to_string() + REFLEXION_FILE) 
        .map_err(ApplicationError::from)?  
        .lines()  // split the string into an iterator of string slices
        .map(CsvLine::from)
        .map(Reflexion::try_from)
        .collect::<Result<Vec<Reflexion>, ApplicationError>>()?
        .iter()
        .try_for_each(reflexion::service::create)?;

    
    copy_recursively(IMPORT_STORAGE.to_string() + REFLEXION_STORAGE, REFLEXION_STORAGE)
}