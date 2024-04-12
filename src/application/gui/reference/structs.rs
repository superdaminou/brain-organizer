use serde::{Deserialize, Serialize};

use crate::application::reference::{service::get_all, structs::{Reference, Tag}};




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReferenceGui {
    pub id: Option<String>,
    pub titre: String,
    pub url: String,
    pub categorie: String
}


impl From<Reference> for ReferenceGui {
    fn from(value: Reference) -> Self {
        ReferenceGui {
            id: value.id,
            titre: value.titre,
            url: value.url,
            categorie: value.categorie.join(",")
        }
    }
}

impl From<ReferenceGui> for Reference {
    fn from(val: ReferenceGui) -> Self {
        Reference {
            id: val.id,
            titre: val.titre,
            url: val.url,
            categorie: val.categorie.split(',').map(String::from).collect::<Vec<Tag>>()
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct SectionReference {
    pub reference: ReferenceGui,
    pub list_references: Vec<ReferenceGui>,
}

impl SectionReference {
    pub fn new() -> Self {
        Self {
            reference: Reference { id: None, titre: "titre".to_string(), url: "String".to_string(), categorie: vec![] }.into(),
            list_references: get_all().unwrap_or_default().iter().map(|reference| ReferenceGui::from(reference.clone())).collect::<Vec<ReferenceGui>>()
        }
    } 
}