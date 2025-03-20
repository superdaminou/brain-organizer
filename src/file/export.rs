use std::{fs::{create_dir, File}, io::Write, path::Path};

use log::info;

const EXPORT_STORAGE: &str = "./export/";

const CONNECTEUR : Connecteur = Connecteur::LOCAL;

pub enum ModeExport  {
    RIS,
    CSV
}

impl TryFrom<String> for ModeExport {
    type Error = ApplicationError;
    fn try_from(value: String) -> Result<Self, ApplicationError> {
        match value.as_str() {
            "RIS" => Ok(ModeExport::RIS),
            "CSV" => Ok(ModeExport::CSV),
            _ =>  Err(ApplicationError::EnumError("WOLOLO".to_string()))
        }
    }
}

impl ModeExport {
    pub fn extension(&self) -> &str {
        match self {
            ModeExport::RIS => "ris",
            ModeExport::CSV => "csv",
        }
    }
}


use crate::{application_error::ApplicationError, connecteur::Connecteur, file::{lib::{copy_recursively, REFERENCE_FILE, REFLEXION_FILE, STORAGE}, ToCsv, ToRis}, notes::ConnecteurNote, reference::ConnecteurReference};

pub fn export(mode: ModeExport) -> Result<(), ApplicationError> {
    match Path::new(EXPORT_STORAGE).exists() {
        true => info!("Export directory - cleaning files"),
        false => {
            info!("Creating storage directory: {}", EXPORT_STORAGE);
            create_dir(EXPORT_STORAGE)
            .map_err(ApplicationError::from)?;
        }
    }

    export_reference(mode)
    .and_then(|_| export_reflexions())

}

fn export_reference(mode: ModeExport) -> Result<(), ApplicationError> {
    let fichier = format!("{}.{}", REFERENCE_FILE, mode.extension());
    info!("Start exporting reference file: {}", fichier);
    write_file(&fichier, <Connecteur as ConnecteurReference>::get_all(&CONNECTEUR)?.to_ris())
}


fn export_reflexions() -> Result<(), ApplicationError> {
    info!("Start exporting reflexion entries: {}", REFLEXION_FILE);
    write_file(REFLEXION_FILE, <Connecteur as ConnecteurNote>::get_all(&CONNECTEUR)?.to_csv())
        .and_then(|_|
            copy_recursively(STORAGE, EXPORT_STORAGE.to_string() + STORAGE).map_err(ApplicationError::from))
}

fn write_file(file: &str, content: String) -> Result<(), ApplicationError>{
    File::create(EXPORT_STORAGE.to_string() + file)
        .and_then(|mut f|f.write_all(content.as_bytes()))
        .map_err(ApplicationError::FileWriteError)
}