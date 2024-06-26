use std::{fs::{self, create_dir}, path::Path};
use log::info;
use crate::application::{graph::lib::{DATABASE_NAME, IDENTIFIER, NODE_TYPE}, reflexion::Reflexion};

use anyhow::Result;

pub const REFLEXION_STORAGE: &str = "./storage/";
pub const REFLEXION_FILE: &str = "reflexion.csv";
pub const NODES_FILE: &str = "nodes.csv";
pub const RELATIONS_FILE: &str = "relations.csv";
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
    
    match Path::new(DATABASE_NAME).exists() {
        true => {
            info!("Graph database already exists");
            Ok(())
        },
        false => {
            info!("Creating graph database directory: {}", DATABASE_NAME);
            let db = indradb::RocksdbDatastore::new_db(DATABASE_NAME)?;
            db.index_property(indradb::Identifier::new(IDENTIFIER)?)?;
            db.index_property(indradb::Identifier::new(NODE_TYPE)?)?;
            Ok(())
        }
    }
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