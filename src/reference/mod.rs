use std::collections::HashSet;

use structs::reference::Reference;
use strum_macros::Display;
pub use tag::Tag;
use uuid::Uuid;

use crate::application_error::ApplicationError;

pub mod connecteur;
pub mod structs;
mod tag;


#[derive(serde::Deserialize, serde::Serialize, Default, Display, Clone, Copy, PartialEq, Eq)]
pub enum ModeTags {
    #[default]
    #[strum(to_string = "Ouvert")]
    OUVERT,
    #[strum(to_string = "Fermé")]
    FERME
}

pub trait ConnecteurReference {
    fn create(&self, entity: &Reference) -> Result<(),ApplicationError>;
    fn get_one(&self, id: &Uuid) -> Result<Reference, ApplicationError>;
    fn get_all(&self,) -> Result<Vec<Reference>, ApplicationError>;
    fn delete(&self, entity_id: &Uuid) -> Result<(), ApplicationError>;
    fn update(&self, entity: &Reference) -> Result<(), ApplicationError>;
    fn search(&self, name: Option<&String>, tags: &HashSet<Tag>, mode: ModeTags) -> Result<Vec<Reference>, ApplicationError>;
    fn all_tags_distinct(&self) -> Result<Vec<Tag>, ApplicationError>;
}