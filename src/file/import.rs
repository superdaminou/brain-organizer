use std::fs::read_to_string;

use log::{error, info};


use crate::{application_error::ApplicationError, connecteur::Connecteur, file::lib::{copy_recursively, REFERENCE_FILE, REFLEXION_FILE, STORAGE}, notes::{ConnecteurNote, Note}, reference::{ structs::reference::Reference, ConnecteurReference}};

const IMPORT_STORAGE: &str = "./import/";

const CONNECTEUR: Connecteur = Connecteur::LOCAL;


pub fn import() -> Result<(), ApplicationError> {
    import_reference()
    .and_then(|_| import_reflexion())
}

fn import_reference() -> Result<(), ApplicationError> {
    info!("Start importing reference file: {}", REFERENCE_FILE);
    read_to_string(IMPORT_STORAGE.to_string() + REFERENCE_FILE).map_err(ApplicationError::from)?
        .lines()  
        .map(Reference::try_from)
        .collect::<Result<Vec<Reference>, ApplicationError>>()?
        .iter()
        .map(|e|<Connecteur as ConnecteurReference>::create(&CONNECTEUR, e))
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
    read_to_string(IMPORT_STORAGE.to_string() + REFLEXION_FILE).map_err(ApplicationError::from)?  
        .lines()  
        .map(Note::try_from)
        .collect::<Result<Vec<Note>, ApplicationError>>()?
        .iter()
        .map(|n | <Connecteur as ConnecteurNote>::create(&CONNECTEUR, n))
        .for_each(|result| {
            match result {
                Ok(()) => (),
                Err(e) => error!("error while inserting reflexion: {}", e)
            }
        });
    copy_recursively(IMPORT_STORAGE.to_string() + STORAGE, STORAGE).map_err(ApplicationError::from)
}
