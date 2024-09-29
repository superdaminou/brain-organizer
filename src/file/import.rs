use std::fs::read_to_string;

use log::{error, info};

use anyhow::{Context,Result};

use crate::{database::CRUD, error::ApplicationError, file::lib::{copy_recursively, REFERENCE_FILE, REFLEXION_FILE, STORAGE}, reference::structs::reference::Reference, reflexion::{service::ReflexionDatabase, Reflexion}};

const IMPORT_STORAGE: &str = "./import/";


pub fn import() -> Result<(), ApplicationError> {
    import_reference()
    .and_then(|_| import_reflexion())
}

fn import_reference() -> Result<(), ApplicationError> {
    info!("Start importing reference file: {}", REFERENCE_FILE);
    read_to_string(IMPORT_STORAGE.to_string() + REFERENCE_FILE).context("Reading file")?
        .lines()  
        .map(Reference::try_from)
        .collect::<Result<Vec<Reference>, ApplicationError>>()?
        .iter()
        .map(Reference::create)
        .for_each(|result| {
            match result {
                Ok(()) => (),
                Err(e) => error!("error while inserting reference: {}", e)
            }
        });
    Ok(())
}

fn import_reflexion() -> Result<(), ApplicationError> {
    info!("Start importing reflexion file: {}", REFLEXION_FILE);
    read_to_string(IMPORT_STORAGE.to_string() + REFLEXION_FILE).context("Read file")?  
        .lines()  
        .map(Reflexion::try_from)
        .collect::<Result<Vec<Reflexion>, ApplicationError>>()?
        .iter()
        .map(Reflexion::create)
        .for_each(|result| {
            match result {
                Ok(()) => (),
                Err(e) => error!("error while inserting reflexion: {}", e)
            }
        });
    copy_recursively(IMPORT_STORAGE.to_string() + STORAGE, STORAGE).map_err(ApplicationError::Other)
}
