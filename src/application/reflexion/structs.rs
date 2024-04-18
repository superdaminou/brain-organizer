use serde::{Deserialize, Serialize};

use crate::application::file::lib::REFLEXION_STORAGE;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reflexion {
    pub id: Option<String>,
    pub sujet: String
}

impl Reflexion {
    pub fn get_path(&self) -> String {
        return REFLEXION_STORAGE.to_string() + self.sujet.trim() + ".txt";
    }

    pub fn new() -> Self {
        Reflexion {
            id: None,
            sujet: String::from("Nouveau sujet")
        }
    }
}
