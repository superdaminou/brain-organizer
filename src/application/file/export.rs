use std::{fs::{create_dir, File}, io::Write, path::Path};

use log::info;

use crate::application::{error::ApplicationError, file::lib::{copy_recursively, REFERENCE_FILE, REFLEXION_FILE, REFLEXION_STORAGE}, reference::{self, structs::reference::Reference}, reflexion::{self, structs::Reflexion}};

const EXPORT_STORAGE: &str = "./export/";
use anyhow::{Context, Result};

pub fn export() -> Result<(), ApplicationError> {
    match Path::new(EXPORT_STORAGE).exists() {
        true => info!("Export directory - cleaning files"),
        false => {
            info!("Creating storage directory: {}", EXPORT_STORAGE);
            create_dir(EXPORT_STORAGE)
            .context("Failed to create dir")?;
        }
    }

    export_reference()
    .and_then(|_| export_reflexions())

}

fn export_reference() -> Result<(), ApplicationError> {
    info!("Start exporting reference file: {}", REFERENCE_FILE);
    let mut references_file = File::create(EXPORT_STORAGE.to_string() + REFERENCE_FILE).map_err(ApplicationError::FileWriteError)?;
    let content = reference::service::get_all()?
        .iter()
        .map(Reference::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");
    
    references_file.write_all(content.as_bytes()).map_err(ApplicationError::FileWriteError)
}

fn export_reflexions() -> Result<(), ApplicationError> {
    info!("Start exporting reflexion entries: {}", REFLEXION_FILE);
    let mut reflexion_file = File::create(EXPORT_STORAGE.to_string() + REFLEXION_FILE).map_err(ApplicationError::FileWriteError)?;
    let content = reflexion::service::get_all()?
        .iter()
        .map(Reflexion::to_csv)
        .collect::<Vec<String>>()
        .join("\r\n");

    reflexion_file.write_all(content.as_bytes()).map_err(ApplicationError::FileWriteError)?;
    copy_recursively(REFLEXION_STORAGE, EXPORT_STORAGE.to_string() + REFLEXION_STORAGE).map_err(ApplicationError::Other)
}