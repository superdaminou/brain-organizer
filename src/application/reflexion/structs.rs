use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::{error::ApplicationError, reference::structs::reference::CsvLine};


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

    /// Initialize a new Reflexion
    /// # Examples
    /// ```
    /// assert_eq!(Reflexion::new(), Reflexion {id: None, sujet: "Nouveau sujet".to_string()});
    /// ```
    pub fn new() -> Self {
        Reflexion {
            id: None,
            sujet: String::from("Nouveau sujet")
        }
    }
}


impl TryFrom<CsvLine> for Reflexion {
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
    fn try_from(value: CsvLine) -> Result<Self, ApplicationError> {
        let split = value.split(';').map(String::from).collect::<Vec<String>>();

        let sujet = split.get(0).ok_or(ApplicationError::from("Missing sujet"))?;

        Ok(Reflexion {
            id: Some(Uuid::new_v4().to_string()),
            sujet: sujet.clone()
        })
    }
    
    type Error = ApplicationError;
}

impl TryFrom<&str> for Reflexion {
    fn try_from(value: &str) -> Result<Self, ApplicationError> {
        Ok(Reflexion {
            id: Some(Uuid::new_v4().to_string()),
            sujet: value.to_string()
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_reflexion() {
        assert_eq!(Reflexion::new(), Reflexion {id: None, sujet: "Nouveau sujet".to_string()});
    }

    #[test]
    fn init_reflexion_from_csv_line() -> Result<(), ApplicationError> {
        assert_eq!(Reflexion::try_from("Un Sujet;")?.sujet, "Un Sujet");
        Ok(())
    }

    #[test]
    fn init_reflexion_from_empty_csv_line() {
        assert!(Reflexion::try_from("").is_err(), "Should be missing subject");
    }

    #[test]
    fn get_path() {
        let reflexion = Reflexion::try_from("t';e,\"()a").unwrap();
        assert_eq!(reflexion.filename(), String::from("tea.txt"));
    }
}