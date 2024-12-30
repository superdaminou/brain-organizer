use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum  Connecteur {
    WEB,
    #[default]
    LOCAL
}

impl Connecteur {
    pub fn from_str(value: &str) -> Connecteur{
        match value {
            "LOCAL" => Connecteur::LOCAL,
            "WEB" => Connecteur::WEB,
            _ => {
                warn!("Mode '{}' non reconnu, mode LOCAL par defaut", value);
                Connecteur::LOCAL
            }
        }
    }
}
