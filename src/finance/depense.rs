use serde::{Deserialize, Serialize};
use strum_macros::Display;
use uuid::Uuid;


#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Depense {
    pub id:Option<Uuid>,
    pub libelle: String,
    pub montant: f32,
    pub repetition: REPETITION
} 

#[derive(Default, Debug, Display, Clone, Serialize, Deserialize)]
pub enum REPETITION {
    ANNUEL,
    #[default]
    MENSUEL,
    JOURNALIER
}

