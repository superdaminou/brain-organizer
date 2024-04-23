use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::{error::ApplicationError, file::lib::REFLEXION_STORAGE, reference::structs::reference::CsvLine};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reflexion {
    pub id: Option<String>,
    pub sujet: String
}

impl Reflexion {
    pub fn get_path(&self) -> String {
        let clean_path = &self.sujet.trim().replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "")
            .split_ascii_whitespace()
            .map(String::from)
            .collect::<Vec<String>>().join("_");
        REFLEXION_STORAGE.to_string() + clean_path + ".txt"
    }

    pub fn new() -> Self {
        Reflexion {
            id: None,
            sujet: String::from("Nouveau sujet")
        }
    }
}


impl TryFrom<CsvLine> for Reflexion {
    fn try_from(value: CsvLine) -> Result<Self, ApplicationError> {
        let split = value.split(';').map(String::from).collect::<Vec<String>>();

        let sujet = split.get(1).ok_or(ApplicationError::from("Missing sujet"))?;

        Ok(Reflexion {
            id: Some(Uuid::new_v4().to_string()),
            sujet: sujet.clone()
        })
    }
    
    type Error = ApplicationError;
}

impl TryFrom<&str> for Reflexion {
    fn try_from(value: &str) -> Result<Self, ApplicationError> {
        let split = value.split(';').map(String::from).collect::<Vec<String>>();

        let sujet = split.get(1).ok_or(ApplicationError::from("Missing sujet"))?;

        Ok(Reflexion {
            id: Some(Uuid::new_v4().to_string()),
            sujet: sujet.clone()
        })
    }
    
    type Error = ApplicationError;
}

impl ToString for Reflexion {
    fn to_string(&self) -> String {
        self.sujet.to_string()
    }
}

impl Reflexion {
    pub fn to_csv(&self) -> String {
        self.sujet.to_string() + ";"
    }
}