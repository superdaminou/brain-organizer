use std::fmt;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    pub id: Option<String>,
    pub titre: String,
    pub url: String,
    pub categorie: Vec<Tag>
}


pub type CsvLine = String;

impl From<CsvLine> for Reference {
    fn from(value: CsvLine) -> Self {
        let split = value.split(';').map(String::from).collect::<Vec<String>>();

        Reference {
            id: Some(Uuid::new_v4().to_string()),
            titre: split.first().expect("Missing title").to_string(),
            categorie: split.get(1).expect("Missing tag").split(',').map(String::from).map(Tag::from).collect(),
            url: split.get(2).expect("Missing url").to_string()
        }
    }
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


#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, PartialEq, Eq)]
pub enum Tag {
    PHILOSOPHIE,
    INFORMATIQUE,
    SOCIOLOGIE,
    POLITIQUE,
    HISTOIRE
}


impl From<String> for Tag {
    fn from(value: std::string::String) -> Self {
        match value.to_lowercase().as_str() {
            "philosophie" => Tag::PHILOSOPHIE,
            "histoire" => Tag::HISTOIRE,
            "informatique" => Tag::INFORMATIQUE,
            "politique" => Tag::POLITIQUE,
            "sociologie" => Tag::SOCIOLOGIE,
            _ => panic!("Nooo")
        }
    }
}


impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        match value.to_string().to_lowercase().as_str() {
            "philosophie" => Tag::PHILOSOPHIE,
            "histoire" => Tag::HISTOIRE,
            "informatique" => Tag::INFORMATIQUE,
            "politique" => Tag::POLITIQUE,
            "sociologie" => Tag::SOCIOLOGIE,
            _ => panic!()
        }
    }
}




impl From<&String> for Tag {
    fn from(value: &std::string::String) -> Self {
        match value.to_lowercase().as_str() {
            "philosophie" => Tag::PHILOSOPHIE,
            "histoire" => Tag::HISTOIRE,
            "informatique" => Tag::INFORMATIQUE,
            "politique" => Tag::POLITIQUE,
            "sociologie" => Tag::SOCIOLOGIE,
            _ => panic!("NOOOOO")
        }
    }
}
impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tag::PHILOSOPHIE => write!(f, "Philosophie"),
            Tag::HISTOIRE => write!(f,"Histoire"),
            Tag::INFORMATIQUE => write!(f, "Informatique"),
            Tag::POLITIQUE => write!(f, "Politique"),
            Tag::SOCIOLOGIE => write!(f, "Sociologie")
        }
    }
}

impl From<Tag> for String {
    fn from(val: Tag) -> Self {
        match val {
            Tag::PHILOSOPHIE => String::from("Philosophie"),
            Tag::HISTOIRE => String::from("Histoire"),
            Tag::INFORMATIQUE => String::from( "Informatique"),
            Tag::POLITIQUE => String::from("Politque"),
            Tag::SOCIOLOGIE => String::from("Sociologie")
        }
    }
}

