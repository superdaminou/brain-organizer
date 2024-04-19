use std::{fs::{create_dir, read_to_string, File}, io::Write, path::Path};
use log::info;
use crate::application::{error::ApplicationError, reference::{self, service::create, structs::Reference}, reflexion::{self, structs::Reflexion}};

pub const REFLEXION_STORAGE: &str = "./storage/";
const REFLEXION_EXPORT: &str = "reflexion.csv";
const REFERENCE_EXPORT: &str = "reference.csv";
const IMPORT_FILE: &str = "import.csv";

pub fn import() -> Result<(), ApplicationError> {
    info!("Start importing reference file: {}", IMPORT_FILE);
    return read_to_string(IMPORT_FILE) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from).try_for_each(|line| create(&Reference::from(line)));
}

pub fn export() -> Result<(), ApplicationError> {
    info!("Start exporting reference file: {}", REFERENCE_EXPORT);
    let mut references_file = File::create(REFERENCE_EXPORT)?;
    let content = reference::service::get_all()?
        .iter()
        .map(Reference::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");
    
    references_file.write_all(content.as_bytes()).map_err(ApplicationError::from)?;
    
    info!("Start exporting reflexion file: {}", REFLEXION_EXPORT);
    let mut reflexion_file = File::create(REFLEXION_EXPORT)?;
    let content = reflexion::service::get_all()?
        .iter()
        .map(Reflexion::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");

    return reflexion_file.write_all(content.as_bytes()).map_err(ApplicationError::from);
}


pub fn ensuring_storage() -> Result<(),ApplicationError> {
    info!("Ensuring storage presence");
    match Path::new(REFLEXION_STORAGE).exists() {
        true => info!("Storage directory present - skipping creation"),
        false => {
            info!("Creating storage directory: {}", REFLEXION_STORAGE);
            create_dir(REFLEXION_STORAGE)?
        }
    }
    Ok(())
}