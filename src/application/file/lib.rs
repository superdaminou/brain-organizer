use std::{fs::{read_to_string, File}, io::Write};

use crate::application::{error::ApplicationError, reference::{self, service::create, structs::Reference}, reflexion::{self, structs::Reflexion}};

pub const REFLEXION_STORAGE: &str = "./storage/";

pub fn import() -> Result<(), ApplicationError> {
    return read_to_string("import.csv") 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from).try_for_each(|line| create(&Reference::from(line)));
}

pub fn export() -> Result<(), ApplicationError> {
    let mut references_file = File::create("reference.csv")?;
    let content = reference::service::get_all()?
        .iter()
        .map(Reference::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");
    
    references_file.write_all(content.as_bytes()).map_err(ApplicationError::from)?;
    
    let mut reflexion_file = File::create("reflexion.csv")?;
    let content = reflexion::service::get_all()?
        .iter()
        .map(Reflexion::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");

    return reflexion_file.write_all(content.as_bytes()).map_err(ApplicationError::from);
}
