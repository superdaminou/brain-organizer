use connecteur_db::ConnecteurDatabaseReference;
use connecteur_web::ConnecteurWebReference;

use crate::connecteur::Connecteur;

use super::ConnecteurReference;

mod connecteur_db;
mod connecteur_web;



impl ConnecteurReference for Connecteur {
    fn create(&self, entity: &super::structs::reference::Reference) -> anyhow::Result<()> {
        match self {
            Connecteur::WEB =>ConnecteurWebReference::new().create(entity),
            Connecteur::LOCAL => ConnecteurDatabaseReference::new().create(entity),
        }
    }

    fn get_one(&self, id: &uuid::Uuid) -> anyhow::Result<super::structs::reference::Reference> {
        match self {
            Connecteur::WEB => ConnecteurWebReference::new().get_one(id),
            Connecteur::LOCAL => ConnecteurDatabaseReference::new().get_one(id),
        }
    }

    fn get_all(&self, ) -> anyhow::Result<Vec<super::structs::reference::Reference>> {
        match self {
            Connecteur::WEB => ConnecteurWebReference::new().get_all(),
            Connecteur::LOCAL => ConnecteurDatabaseReference::new().get_all(),
        }
    }

    fn delete(&self, entity_id: &uuid::Uuid) -> anyhow::Result<usize> {
        match self {
            Connecteur::WEB=> ConnecteurWebReference::new().delete(entity_id),
            Connecteur::LOCAL => ConnecteurDatabaseReference::new().delete(entity_id),
        }
    }

    fn update(&self, entity: &super::structs::reference::Reference) -> anyhow::Result<()> {
        match self {
            Connecteur::WEB => ConnecteurWebReference::new().update(entity),
            Connecteur::LOCAL => ConnecteurDatabaseReference::new().update(entity),
        }
    }

    fn search(&self, name: Option<&String>, tags: &std::collections::HashSet<super::tag::Tag>, mode: super::ModeTags) -> anyhow::Result<Vec<super::structs::reference::Reference>> {
        match self {
            Connecteur::WEB => ConnecteurWebReference::new().search(name, tags, mode),
            Connecteur::LOCAL => ConnecteurDatabaseReference::new().search(name, tags, mode),
        }
    }
}