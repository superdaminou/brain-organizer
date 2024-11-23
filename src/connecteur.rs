use log::warn;

pub enum  Connecteur {
    WEB,
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