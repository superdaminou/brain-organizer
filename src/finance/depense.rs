use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use uuid::Uuid;

use crate::application_error::ApplicationError;


#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Depense {
    pub id:Option<Uuid>,
    pub libelle: String,
    pub montant: f32,
    pub repetition: REPETITION
} 

#[derive(Default, Debug, Display, Clone, Serialize, Deserialize, EnumIter, PartialEq, Eq)]
pub enum REPETITION {
    ANNUEL,
    #[default]
    MENSUEL,
    JOURNALIER
}

impl Depense {
    pub fn convert(&self, repetition: &REPETITION) -> f32 {
        match (&self.repetition, repetition) {
            (REPETITION::ANNUEL, REPETITION::MENSUEL) => self.montant / 12.0,
            (REPETITION::ANNUEL, REPETITION::JOURNALIER) => self.montant / 365.0,
            (REPETITION::MENSUEL, REPETITION::ANNUEL) => self.montant * 12.0,
            (REPETITION::MENSUEL, REPETITION::JOURNALIER) => self.montant * 12.0 / 365.0,
            (REPETITION::JOURNALIER, REPETITION::ANNUEL) => self.montant * 365.0,
            (REPETITION::JOURNALIER, REPETITION::MENSUEL) => self.montant * 365.0 / 12.0,
            _ => self.montant,
        }
    }
}

impl TryFrom<&str> for REPETITION {
    type Error = ApplicationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            CODE_ANNUEL => Ok(REPETITION::ANNUEL),
            CODE_MENSUEL => Ok(REPETITION::MENSUEL),
            CODE_JOURNALIER => Ok(REPETITION::JOURNALIER),
            _ => Err(ApplicationError::EnumError("Wesh".to_string()))
        }
    }
}

impl REPETITION {
    pub fn code(&self) -> String {
        match self {
            REPETITION::ANNUEL => CODE_ANNUEL.to_string(),
            REPETITION::MENSUEL => CODE_MENSUEL.to_string(),
            REPETITION::JOURNALIER => CODE_JOURNALIER.to_string(),
        }
    }
}

const CODE_ANNUEL : &str = "ANNUEL";
const CODE_MENSUEL : &str = "MENSUEL";
const CODE_JOURNALIER : &str = "JOURNALIER"; 