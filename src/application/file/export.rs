use std::{fs::{create_dir, File}, io::Write, path::Path};

use log::info;

use crate::application::{error::ApplicationError, file::lib::{copy_recursively, REFERENCE_FILE, REFLEXION_FILE, REFLEXION_STORAGE}, reference::{self, structs::reference::Reference}, reflexion::{self, structs::Reflexion}};

const EXPORT_STORAGE: &str = "./export/";

pub fn export() -> Result<(), ApplicationError> {
    match Path::new(EXPORT_STORAGE).exists() {
        true => info!("Export directory - cleaning files"),
        false => {
            info!("Creating storage directory: {}", EXPORT_STORAGE);
            create_dir(EXPORT_STORAGE)?
        }
    }


    info!("Start exporting reference file: {}", REFERENCE_FILE);
    let mut references_file = File::create(EXPORT_STORAGE.to_string() + REFERENCE_FILE)?;
    let content = reference::service::get_all()?
        .iter()
        .map(Reference::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");
    
    references_file.write_all(content.as_bytes()).map_err(ApplicationError::from)?;
    
    info!("Start exporting reflexion entries: {}", REFLEXION_FILE);
    let mut reflexion_file = File::create(EXPORT_STORAGE.to_string() + REFLEXION_FILE)?;
    let content = reflexion::service::get_all()?
        .iter()
        .map(Reflexion::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");

    reflexion_file.write_all(content.as_bytes()).map_err(ApplicationError::from)?;


    copy_recursively(REFLEXION_STORAGE, EXPORT_STORAGE.to_string() + REFLEXION_STORAGE)
}