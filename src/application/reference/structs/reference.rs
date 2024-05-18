use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::application::{error::ApplicationError, file::ToCsv};

use super::tag::Tag;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    pub id: Option<String>,
    pub titre: String,
    pub url: String,
    pub tags: Vec<Tag>,
    pub date_creation: NaiveDate<>
}


pub type CsvLine = String;
const SEPARATOR : &str = ";";


impl TryFrom<CsvLine> for Reference {
    fn try_from(value: CsvLine) -> Result<Self, Self::Error> {
        let split = value.split(SEPARATOR).map(String::from).collect::<Vec<String>>();

        let categorie = split.get(1)
            .expect("Missing tag")
            .split('\\')
            .map(Tag::try_from)
            .collect::<Result<Vec<Tag>, ApplicationError>>()?;

        Ok(Reference {
            id: Some(Uuid::new_v4().to_string()),
            titre: split.first().expect("Missing title").to_string(),
            tags: categorie,
            url: split.get(2).expect("Missing url").to_string(),
            date_creation: Local::now().date_naive()
        })
    }
    
    type Error = ApplicationError;
}


impl TryFrom<&str> for Reference {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split(SEPARATOR).map(String::from).collect::<Vec<String>>();

        let categorie = split.get(1)
            .expect("Missing tag")
            .split('\\')
            .map(Tag::try_from)
            .collect::<Result<Vec<Tag>, ApplicationError>>()?;

        Ok(Reference {
            id: Some(Uuid::new_v4().to_string()),
            titre: split.first().expect("Missing title").to_string(),
            tags: categorie,
            url: split.get(2).expect("Missing url").to_string(),
            date_creation: Local::now().date_naive()
        })
    }
    
    type Error = ApplicationError;
}


impl Default for Reference {
    fn default() -> Self {
        Self {
            tags: vec![],
            id: None,
            titre: String::from("Reference"),
            url: String::from("www.url.com"),
            date_creation: Local::now().date_naive()
        }
    }
}

impl ToCsv for Reference {
    fn to_csv(&self) -> String {
        self.titre.to_string() + SEPARATOR + &self.tags.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("\\") + SEPARATOR + &self.url.to_string()
    }
}

impl ToCsv for Vec<Reference> {
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
    fn to_csv() {
        let mut r = Reference::default();
        r.tags = vec![Tag::Histoire, Tag::Informatique];
        assert_eq!(r.to_csv(), "Reference;Histoire\\Informatique;www.url.com");
    }

    #[test]
    fn to_csv_vec() {
        let first_r = Reference { titre: "UnAutreTitre".to_string(), tags: vec![Tag::Histoire, Tag::Informatique], ..Default::default() };
        let  second_r = Reference { tags: vec![Tag::Sociologie, Tag::Philosophie], ..Default::default() };
        assert_eq!(vec![first_r, second_r].to_csv(), "UnAutreTitre;Histoire\\Informatique;www.url.com\\Reference;Sociologie\\Philosophie;www.url.com");
    }

    #[test]
    fn reference_from_string() {
        let r = Reference::try_from("Nom;Histoire\\Informatique;www.url.com".to_string()).unwrap();
        assert_eq!(r.tags.len(), 2);
        assert_eq!(r.titre, "Nom");
        assert_eq!(r.url, "www.url.com");
    }

    #[test]
    fn reference_from_str() {
        let r = Reference::try_from("Nom;Histoire\\Informatique;www.url.com").unwrap();
        assert_eq!(r.tags.len(), 2);
        assert_eq!(r.titre, "Nom");
        assert_eq!(r.url, "www.url.com");
    }

    #[test]
    fn reference_default() {
        let r = Reference::default();
        assert_eq!(r.tags.len(), 0);
        assert_eq!(r.titre, "Reference");
        assert_eq!(r.url, "www.url.com");
    }
}