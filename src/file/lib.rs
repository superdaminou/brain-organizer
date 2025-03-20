use std::{fs::{self, create_dir}, path::Path};
use log::info;

use crate::application_error::ApplicationError;

pub const STORAGE: &str = "./storage/";
pub const REFLEXION_FILE: &str = "reflexion.csv";
pub const REFERENCE_FILE: &str = "reference.ris";


pub fn ensuring_storage() -> Result<(), ApplicationError> {
    info!("Ensuring storage presence");
    match Path::new(STORAGE).exists() {
        true => {
            info!("Storage directory present - skipping creation");
            Ok(())
        },
        false => {
            info!("Creating storage directory: {}", STORAGE);
            Ok(create_dir(STORAGE)?)
        }
    }

}

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<(), ApplicationError> {
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

pub fn construct_path(filename: &String) -> String {
    STORAGE.to_string() + filename
}