use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reflexion {
    pub id: Option<String>,
    pub sujet: String,
    pub contenu: String
}

