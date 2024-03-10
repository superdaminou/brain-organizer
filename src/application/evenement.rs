use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Evenement {
    pub id: Option<u32>,
    pub titre: String,
    pub niveau: String
}