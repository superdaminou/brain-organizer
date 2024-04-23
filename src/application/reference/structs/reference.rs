use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::application::error::ApplicationError;

use super::tag::Tag;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    pub id: Option<String>,
    pub titre: String,
    pub url: String,
    pub categorie: Vec<Tag>
}


pub type CsvLine = String;



impl TryFrom<CsvLine> for Reference {
    fn try_from(value: CsvLine) -> Result<Self, Self::Error> {
        let split = value.split(';').map(String::from).collect::<Vec<String>>();

        let categorie = split.get(1)
            .expect("Missing tag")
            .split('\\')
            .map(Tag::try_from)
            .collect::<Result<Vec<Tag>, ApplicationError>>()?;

        Ok(Reference {
            id: Some(Uuid::new_v4().to_string()),
            titre: split.first().expect("Missing title").to_string(),
            categorie,
            url: split.get(2).expect("Missing url").to_string()
        })
    }
    
    type Error = ApplicationError;
}


impl TryFrom<&str> for Reference {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split(';').map(String::from).collect::<Vec<String>>();

        let categorie = split.get(1)
            .expect("Missing tag")
            .split('\\')
            .map(Tag::try_from)
            .collect::<Result<Vec<Tag>, ApplicationError>>()?;

        Ok(Reference {
            id: Some(Uuid::new_v4().to_string()),
            titre: split.first().expect("Missing title").to_string(),
            categorie,
            url: split.get(2).expect("Missing url").to_string()
        })
    }
    
    type Error = ApplicationError;
}



impl ToString for Reference {
    fn to_string(&self) -> String {
        self.titre.to_string() + &self.categorie.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("\\") + &self.url.to_string()
    }
}

impl Reference {
    pub fn to_csv(&self) -> String {
        self.titre.to_string() + ";" + &self.categorie.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("\\") + ";" + &self.url.to_string()
    }
    pub fn new() -> Self {
        Self {
            categorie: vec![],
            id: None,
            titre: String::from("Reference"),
            url: String::from("www.url.com")
        }
    }
}