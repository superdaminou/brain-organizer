use std::{fs::{self, create_dir}, path::Path};
use log::info;
use crate::application::reflexion::structs::Reflexion;

use anyhow::Result;

pub const REFLEXION_STORAGE: &str = "./storage/";
pub const REFLEXION_FILE: &str = "reflexion.csv";
pub const REFERENCE_FILE: &str = "reference.csv";


pub fn ensuring_storage() -> Result<()> {
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

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
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

pub fn construct_path(reflexion: &Reflexion) -> String {
    REFLEXION_STORAGE.to_string() + &reflexion.filename()
}