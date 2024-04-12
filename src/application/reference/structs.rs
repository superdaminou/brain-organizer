use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::CsvLine;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    pub id: Option<String>,
    pub titre: String,
    pub url: String,
    pub categorie: Vec<Tag>
}

pub type Tag = String;


impl From<CsvLine> for Reference {
    fn from(value: CsvLine) -> Self {
        let split = value.split(';').map(String::from).collect::<Vec<String>>();

        Reference {
            id: Some(Uuid::new_v4().to_string()),
            titre: split.first().expect("Missing title").to_string(),
            categorie: split.get(1).expect("Missing tag").split(',').map(String::from).collect(),
            url: split.get(2).expect("Missing url").to_string()
        }
    }
}

impl ToString for Reference {
    fn to_string(&self) -> String {
        return self.titre.to_string() + &self.categorie.join("\\") + &self.url.to_string();
    }
}

impl Reference {
    pub fn to_csv(&self) -> String {
        return self.titre.to_string() + ";" + &self.categorie.join("\\") + ";" + &self.url.to_string();
    }
}
 