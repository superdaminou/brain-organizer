use std::{fs::{self, create_dir, read_to_string, File}, io::Write, path::Path};
use log::info;
use crate::application::{error::ApplicationError, reference::{self, structs::reference::Reference}, reflexion::{self, structs::Reflexion}};

pub const REFLEXION_STORAGE: &str = "./storage/";
const REFLEXION_EXPORT: &str = "reflexion.csv";
const REFERENCE_EXPORT: &str = "reference.csv";
const EXPORT_STORAGE: &str = "./export/";
const IMPORT_STORAGE: &str = "./import/";

pub fn import() -> Result<(), ApplicationError> {
    info!("Start importing reference file: {}", REFERENCE_EXPORT);
    read_to_string(IMPORT_STORAGE.to_string() + REFERENCE_EXPORT) 
        .map_err(ApplicationError::from)?
        .lines()  // split the string into an iterator of string slices
        .map(Reference::try_from)
        .collect::<Result<Vec<Reference>, ApplicationError>>()?
        .iter()
        .try_for_each(reference::service::create)?;

    info!("Start importing reflexion file: {}", REFLEXION_EXPORT);
    read_to_string(IMPORT_STORAGE.to_string() + REFLEXION_EXPORT) 
        .map_err(ApplicationError::from)?  
        .lines()  // split the string into an iterator of string slices
        .map(Reflexion::try_from)
        .collect::<Result<Vec<Reflexion>, ApplicationError>>()?
        .iter()
        .try_for_each(reflexion::service::create)?;

    
    copy_recursively(IMPORT_STORAGE.to_string() + REFLEXION_STORAGE, REFLEXION_STORAGE)
}

pub fn export() -> Result<(), ApplicationError> {
    match Path::new(EXPORT_STORAGE).exists() {
        true => info!("Export directory - cleaning files"),
        false => {
            info!("Creating storage directory: {}", EXPORT_STORAGE);
            create_dir(EXPORT_STORAGE)?
        }
    }


    info!("Start exporting reference file: {}", REFERENCE_EXPORT);
    let mut references_file = File::create(EXPORT_STORAGE.to_string() + REFERENCE_EXPORT)?;
    let content = reference::service::get_all()?
        .iter()
        .map(Reference::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");
    
    references_file.write_all(content.as_bytes()).map_err(ApplicationError::from)?;
    
    info!("Start exporting reflexion entries: {}", REFLEXION_EXPORT);
    let mut reflexion_file = File::create(EXPORT_STORAGE.to_string() + REFLEXION_EXPORT)?;
    let content = reflexion::service::get_all()?
        .iter()
        .map(Reflexion::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");

    reflexion_file.write_all(content.as_bytes()).map_err(ApplicationError::from)?;


    copy_recursively(REFLEXION_STORAGE, EXPORT_STORAGE.to_string() + REFLEXION_STORAGE)
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

fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<(), ApplicationError> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}