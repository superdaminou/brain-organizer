use std::collections::HashSet;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application_error::ApplicationError, file::ToCsv, reference::{ tag::Tag}};



#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Reference {
    pub id: Option<String>,
    pub titre: String,
    pub url: String,
    pub tags: HashSet<Tag>,
    pub date_creation: NaiveDate<>,
    pub to_read: bool
}


pub type CsvLine = str;
const SEPARATOR : &str = ";";


impl TryFrom<&str> for Reference {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split(SEPARATOR).map(String::from).collect::<Vec<String>>();

        let categorie = split.get(1)
            .expect("Missing tag")
            .split('\\')
            .map(str::to_string)
            .map(Tag)
            .collect::<HashSet<Tag>>();

        Ok(Reference {
            id: Some(Uuid::new_v4().to_string()),
            titre: split.first().expect("Missing title").to_string(),
            tags: categorie,
            url: split.get(2).expect("Missing url").to_string(),
            date_creation: Local::now().date_naive(),
            to_read: true
        })
    }
    
    type Error = ApplicationError;
}


impl Default for Reference {
    fn default() -> Self {
        Self {
            tags: HashSet::new(),
            id: None,
            titre: String::from("Reference"),
            url: String::from("www.url.com"),
            date_creation: Local::now().date_naive(),
            to_read: false
        }
    }
}

impl ToCsv for Reference {
    fn to_csv(&self) -> String {
        let mut tags = self.tags.iter().map(|t| t.0.clone()).collect::<Vec<String>>();
        tags.sort();
        self.titre.to_string() + SEPARATOR + &tags.join("\\") + SEPARATOR + &self.url.to_string()
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
        let r = Reference{ tags:  HashSet::from([Tag("Histoire".to_string()), Tag("Informatique".to_string())]), ..Default::default()};
        assert_eq!(r.to_csv(), "Reference;Histoire\\Informatique;www.url.com");
    }

    #[test]
    fn to_csv_vec() {
        let first_r = Reference { titre: "UnAutreTitre".to_string(), tags: HashSet::from([Tag("Informatique".to_string()), Tag("Histoire".to_string())]), ..Default::default() };
        let  second_r = Reference { tags: HashSet::from([Tag("Philosophie".to_string()), Tag("Sociologie".to_string())]), ..Default::default() };
        assert_eq!(vec![first_r, second_r].to_csv(), "UnAutreTitre;Histoire\\Informatique;www.url.com\r\nReference;Philosophie\\Sociologie;www.url.com");
    }

    #[test]
    fn reference_from_string() {
        let r = Reference::try_from("Nom;Histoire\\Informatique;www.url.com").unwrap();
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