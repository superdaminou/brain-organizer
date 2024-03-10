use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contenu {
    pub id: Option<u32>,
    pub titre: String,
    pub url: String,
    pub categorie: String
}