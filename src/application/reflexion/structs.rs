use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::{error::ApplicationError, file::ToCsv, reference::structs::reference::CsvLine};


const DELIMITER : &str = ";"; 

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Reflexion {
    pub id: Option<String>,
    pub sujet: String
}

impl Reflexion {
    /// Construct filename from subject
    /// # Examples
    /// ```
    /// assert_eq!(Reflexion::new().get_path(), "Nouveau_sujet.txt".to_string());
    /// ```
    pub fn filename(&self) -> String {
        let clean_path = &self.sujet.trim().replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "")
            .split_ascii_whitespace()
            .map(String::from)
            .collect::<Vec<String>>().join("_");
        clean_path.to_string() + ".txt"
    }
}

impl Default for Reflexion {
    fn default() -> Self {
        Self { 
            id: None,
            sujet: String::from("Nouveau sujet")
         }
    }
}


impl TryFrom<&CsvLine> for Reflexion {
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

        let sujet = split.first().ok_or(ApplicationError::DefaultError)?;

        if sujet.is_empty() {
            return Err(ApplicationError::DefaultError);
        }

        Ok(Reflexion {
            id: Some(Uuid::new_v4().to_string()),
            sujet: sujet.clone()
        })
    }
    
    type Error = ApplicationError;
}


impl Display for Reflexion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sujet)
    }
}


impl ToCsv for Reflexion {
    fn to_csv(&self) -> String {
        self.sujet.to_string() + DELIMITER
    }
}

impl ToCsv for Vec<Reflexion> {
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
        assert_eq!(Reflexion::default().to_string(), "Nouveau sujet".to_string());
    }

    #[test]
    fn reflexion_to_csv() {
        assert_eq!(Reflexion::default().to_csv(), "Nouveau sujet;".to_string());
    }

    #[test]
    fn init_reflexion() {
        assert_eq!(Reflexion::default(), Reflexion {id: None, sujet: "Nouveau sujet".to_string()});
    }

    #[test]
    fn init_reflexion_from_csv_line() -> Result<(), ApplicationError> {
        assert_eq!(Reflexion::try_from("Un Sujet;")?.sujet, "Un Sujet");
        Ok(())
    }

    #[test]
    fn init_reflexion_from_empty_csv_line() {
        assert!(Reflexion::try_from("").is_err(), "Should be \"Missing subject\"");
    }
}