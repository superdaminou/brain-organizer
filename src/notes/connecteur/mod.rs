use connecteur_db::ConnecteurNoteDb;
use connecteur_web::ConnecteurWebNote;
use uuid::Uuid;

use crate::{application_error::ApplicationError, connecteur::Connecteur};

use super::{ConnecteurNote, Note};

pub mod connecteur_db;
pub mod connecteur_web;




impl ConnecteurNote for Connecteur {
    fn create(&self, entity: &Note) -> anyhow::Result<()> {
        match self {
            Connecteur::WEB =>ConnecteurWebNote::new().create(entity),
            Connecteur::LOCAL => ConnecteurNoteDb::new().create(entity),
        }
    }

    fn get_one(&self, id: &String) -> anyhow::Result<Note> {
        match self {
            Connecteur::WEB => ConnecteurWebNote::new().get_one(id),
            Connecteur::LOCAL => ConnecteurNoteDb::new().get_one(id),
        }
    }

    fn get_all(&self, ) -> anyhow::Result<Vec<Note>> {
        match self {
            Connecteur::WEB => ConnecteurWebNote::new().get_all(),
            Connecteur::LOCAL => ConnecteurNoteDb::new().get_all(),
        }
    }

    fn delete(&self, entity_id: &String) -> anyhow::Result<()> {
        match self {
            Connecteur::WEB=> ConnecteurWebNote::new().delete(entity_id),
            Connecteur::LOCAL => ConnecteurNoteDb::new().delete(entity_id),
        }
    }

    fn update(&self, entity: &Note) -> Result<(), ApplicationError> {
        match self {
            Connecteur::WEB => ConnecteurWebNote::new().update(entity),
            Connecteur::LOCAL => ConnecteurNoteDb::new().update(entity),
        }
    }

}