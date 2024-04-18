use crate::application::reflexion::{service::get_all, structs::Reflexion};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SectionReflexion {
    pub reflexion: Reflexion,
    pub list_reflexions: Vec<Reflexion>,
    pub edit: EditText,
    pub edit_reflexion: EditReflexion
}

impl SectionReflexion {
    pub fn new() -> Self {
        Self {
            reflexion: Reflexion { id: None, sujet: "sujet".to_string() }.into(),
            list_reflexions: get_all().unwrap_or_default(),
            edit: EditText::default(),
            edit_reflexion: EditReflexion {show: false, reflexion: Reflexion::new(), contenu: String::from("")}
            
        }
    } 
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct EditText {}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct EditReflexion {
    pub show: bool,
    pub reflexion: Reflexion,
    pub contenu: String
}