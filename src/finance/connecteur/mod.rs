use connecteur_db::ConnecteurDepenseDb;
use connecteur_web::ConnecteurWebDepense;

use crate::{application_error::ApplicationError, connecteur::Connecteur};

use super::{ConnecteurDepense, Depense};

pub mod connecteur_db;
pub mod connecteur_web;


impl ConnecteurDepense for Connecteur {
    fn create(&self, entity: &Depense) -> Result<(), ApplicationError> {
        match self {
            Connecteur::WEB =>ConnecteurWebDepense::new().create(entity),
            Connecteur::LOCAL => ConnecteurDepenseDb::new().create(entity),
        }
    }

    fn get_one(&self, id: &String) -> Result<Depense, ApplicationError> {
        match self {
            Connecteur::WEB => ConnecteurWebDepense::new().get_one(id),
            Connecteur::LOCAL => ConnecteurDepenseDb::new().get_one(id),
        }
    }

    fn get_all(&self, ) -> Result<Vec<Depense>, ApplicationError> {
        match self {
            Connecteur::WEB => ConnecteurWebDepense::new().get_all(),
            Connecteur::LOCAL => ConnecteurDepenseDb::new().get_all(),
        }
    }

    fn delete(&self, entity_id: &String) -> Result<(), ApplicationError> {
        match self {
            Connecteur::WEB=> ConnecteurWebDepense::new().delete(entity_id),
            Connecteur::LOCAL => ConnecteurDepenseDb::new().delete(entity_id),
        }
    }

    fn update(&self, entity: &Depense) -> Result<(), ApplicationError> {
        match self {
            Connecteur::WEB => ConnecteurWebDepense::new().update(entity),
            Connecteur::LOCAL => ConnecteurDepenseDb::new().update(entity),
        }
    }

}