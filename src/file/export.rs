use std::{fs::{create_dir, File}, io::Write, path::Path};

use log::info;

const EXPORT_STORAGE: &str = "./export/";
use anyhow::{Context, Result};

const CONNECTEUR : Connecteur = Connecteur::LOCAL;

use crate::{application_error::ApplicationError, connecteur::Connecteur, file::{lib::{copy_recursively, REFERENCE_FILE, REFLEXION_FILE, STORAGE}, ToCsv}, notes::ConnecteurNote, reference::ConnecteurReference};

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
    write_file(REFERENCE_FILE, <Connecteur as ConnecteurReference>::get_all(&CONNECTEUR)?.to_csv())
}


fn export_reflexions() -> Result<(), ApplicationError> {
    info!("Start exporting reflexion entries: {}", REFLEXION_FILE);
    write_file(REFLEXION_FILE, <Connecteur as ConnecteurNote>::get_all(&CONNECTEUR)?.to_csv())
        .and_then(|_|
            copy_recursively(STORAGE, EXPORT_STORAGE.to_string() + STORAGE).map_err(ApplicationError::Other))
}

fn write_file(file: &str, content: String) -> Result<(), ApplicationError>{
    File::create(EXPORT_STORAGE.to_string() + file)
        .and_then(|mut f|f.write_all(content.as_bytes()))
        .map_err(ApplicationError::FileWriteError)
}