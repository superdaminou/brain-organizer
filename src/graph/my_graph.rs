use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application_error::ApplicationError, connecteur::Connecteur, gui::{EditableFile, Fileable}};

use super::ConnecteurGraph;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub id: Uuid,
    pub filename: String,
    pub contenu: String
}

impl Fileable for Graph {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn filename(&self) -> String {
        self.filename.clone()
    }

    fn contenu(&self, connecteur: &Connecteur) -> String {
        connecteur.get_one(&self.id.to_string())
            .map(|n|n.contenu)
            .unwrap_or_else(|e|e.to_string())
    }

    fn write(file: &EditableFile, connecteur: &Connecteur) -> Result<(), ApplicationError> {
        let graph = Graph {
            contenu: file.contenu.clone(),
            id: Uuid::parse_str(&file.id).unwrap(),
            filename: file.filename.clone(),
        };
        info!("Updating contenu: {}", &graph.contenu);
        connecteur.update(&graph)
        
    }
    
    fn sujet(&self) -> String {
        self.filename.clone()
    }
}


impl Default for Graph {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            filename: String::default(),
            contenu: String::default()
        }
    }
}

impl From<&String> for Graph {
    fn from(value: &String) -> Self {
        Self {
            filename: value.clone(),
            id: Uuid::new_v4(),
            contenu: String::default()
        }
    }
}



impl Graph {
    pub fn filename(&self) -> String {
        self.filename.clone() + ".dot"
    }
    
}

 
