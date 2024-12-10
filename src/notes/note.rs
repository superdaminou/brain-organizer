use std::{fmt::{format, Display}, fs::read_to_string};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application_error::ApplicationError, connecteur::Connecteur, file::{construct_path, ToCsv}, gui::{EditableFile, Fileable}, reference::structs::reference::CsvLine};

use super::{connecteur::connecteur_db::ConnecteurNoteDb, ConnecteurNote};

const DELIMITER : &str = ";"; 

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Note {
    pub id: String,
    pub sujet: String,
    pub contenu: String
}

impl Fileable for Note {
    fn filename(&self) -> String {
        self.filename()
    }

    fn contenu(&self, connecteur: &Connecteur) -> String {
        connecteur.get_one(&self.id)
            .map(|n|n.contenu)
            .unwrap_or_else(|e|e.to_string())
    }

    fn write(file: &EditableFile, connecteur: &Connecteur) -> Result<(), ApplicationError> {
        let note = Note {
            contenu: file.contenu.clone(),
            id: file.id.clone(),
            sujet: file.sujet.clone(),
        };
        connecteur.update(&note)
    }
    
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn sujet(&self) -> String {
        self.sujet.clone()
    }
}

impl Note {
    /// Construct filename from subject
    /// # Examples
    /// ```
    /// assert_eq!(Note::new().get_path(), "Nouveau_sujet.txt".to_string());
    /// ```
    pub fn filename(&self) -> String {
        let clean_path = &self.sujet.trim().replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "")
            .split_ascii_whitespace()
            .map(String::from)
            .collect::<Vec<String>>().join("_");
        clean_path.to_string() + ".txt"
    }

    pub fn contenu(&self) -> Result<String, std::io::Error> {
        let filename= self.filename();
        read_to_string(construct_path(&filename))
    }
}

impl Default for Note {
    fn default() -> Self {
        Self { 
            id: Uuid::new_v4().to_string(),
            sujet: String::from("Nouveau sujet"),
            contenu: String::default()
         }
    }
}


impl TryFrom<&CsvLine> for Note {
    /// Trying to create Reflexion from a CSV Line
    /// # Examples
    /// ```
    /// assert_eq!(Reflexion::try_from("Un sujet;").sujet, "Un sujet".to_string()});
    /// ```
    ///  # Error
    /// Return an Application Error if it cannot extract a subject from CSV line
    /// ```
    /// assert_eq!(Reflexion::try_from("").is_err())
    /// ```
    fn try_from(value: &CsvLine) -> Result<Self, ApplicationError> {
        let split = value.split(DELIMITER).map(String::from).collect::<Vec<String>>();

        let sujet = split.first().ok_or(ApplicationError::DefaultError("Sujet vide".to_string()))?;

        if sujet.is_empty() {
            return Err(ApplicationError::DefaultError("Sujet vide".to_string()));
        }

        Ok(Note {
            id: Uuid::new_v4().to_string(),
            sujet: sujet.clone(),
            contenu: String::default()
        })
    }
    
    type Error = ApplicationError;
}


impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sujet)
    }
}


impl ToCsv for Note {
    fn to_csv(&self) -> String {
        self.sujet.to_string() + DELIMITER
    }
}

impl ToCsv for Vec<Note> {
    fn to_csv(&self) -> String {
        self.iter()
        .map(|item|item.to_csv())
        .collect::<Vec<String>>()
        .join("\r\n")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflexion_to_string() {
        assert_eq!(Note::default().to_string(), "Nouveau sujet".to_string());
    }

    #[test]
    fn reflexion_to_csv() {
        assert_eq!(Note::default().to_csv(), "Nouveau sujet;".to_string());
    }

    #[test]
    fn init_reflexion() {
        let note_default = Note::default(); 
        assert_eq!(note_default.sujet, "Nouveau sujet");
    }

    #[test]
    fn init_reflexion_from_csv_line() -> Result<(), ApplicationError> {
        assert_eq!(Note::try_from("Un Sujet;")?.sujet, "Un Sujet");
        Ok(())
    }

    #[test]
    fn init_reflexion_from_empty_csv_line() {
        assert!(Note::try_from("").is_err(), "Should be \"Missing subject\"");
    }
}