use connecteur_db::ConnecteurGraphDb;
use connecteur_web::ConnecteurWebGraph;

use crate::{application_error::ApplicationError, connecteur::Connecteur};

use super::{ConnecteurGraph, Graph};

pub mod connecteur_db;
pub mod connecteur_web;

impl ConnecteurGraph for Connecteur {
    fn create(&self, entity: &Graph) -> Result<(), ApplicationError> {
        match self {
            Connecteur::WEB =>ConnecteurWebGraph::new().create(entity),
            Connecteur::LOCAL => ConnecteurGraphDb::new().create(entity),
        }
    }

    fn get_one(&self, id: &String) -> Result<Graph, ApplicationError> {
        match self {
            Connecteur::WEB => ConnecteurWebGraph::new().get_one(id),
            Connecteur::LOCAL => ConnecteurGraphDb::new().get_one(id),
        }
    }

    fn get_all(&self, ) -> Result<Vec<Graph>, ApplicationError> {
        match self {
            Connecteur::WEB => ConnecteurWebGraph::new().get_all(),
            Connecteur::LOCAL => ConnecteurGraphDb::new().get_all(),
        }
    }

    fn delete(&self, entity_id: &String) -> Result<(), ApplicationError> {
        match self {
            Connecteur::WEB=> ConnecteurWebGraph::new().delete(entity_id),
            Connecteur::LOCAL => ConnecteurGraphDb::new().delete(entity_id),
        }
    }

    fn update(&self, entity: &Graph) -> Result<(), ApplicationError> {
        match self {
            Connecteur::WEB => ConnecteurWebGraph::new().update(entity),
            Connecteur::LOCAL => ConnecteurGraphDb::new().update(entity),
        }
    }

}