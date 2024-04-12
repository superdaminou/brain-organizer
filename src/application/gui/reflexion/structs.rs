use serde::{Deserialize, Serialize};

use crate::application::reflexion::{service::get_all, structs::Reflexion};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReflexionGui {
    pub id: Option<String>,
    pub contenu: String,
    pub sujet: String
}


impl From<Reflexion> for ReflexionGui {
    fn from(value: Reflexion) -> Self {
        ReflexionGui {
            id: value.id,
            contenu: value.contenu,
            sujet: value.sujet
        }
    }
}

impl From<ReflexionGui> for Reflexion {
    fn from(val: ReflexionGui) -> Self {
        Reflexion {
            id: val.id,
            contenu: val.contenu,
            sujet: val.sujet
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct SectionReflexion {
    pub reflexion: ReflexionGui,
    pub list_reflexions: Vec<ReflexionGui>,
}

impl SectionReflexion {
    pub fn new() -> Self {
        Self {
            reflexion: Reflexion { id: None, sujet: "sujet".to_string(), contenu: "String".to_string() }.into(),
            list_reflexions: get_all().unwrap_or_default().iter().map(|reflexion| ReflexionGui::from(reflexion.clone())).collect::<Vec<ReflexionGui>>()
        }
    } 
}