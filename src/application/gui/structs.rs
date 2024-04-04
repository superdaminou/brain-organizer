use serde::{Deserialize, Serialize};

use crate::application::reference::reference::Reference;

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
