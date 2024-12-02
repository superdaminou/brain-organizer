use std::collections::HashSet;

use structs::reference::Reference;
use strum_macros::Display;
use anyhow::Result;
use tag::Tag;
use uuid::Uuid;

pub mod connecteur;
pub mod structs;
pub mod tag;



#[derive(serde::Deserialize, serde::Serialize, Default, Display, Clone, Copy, PartialEq, Eq)]
pub enum ModeTags {
    #[default]
    #[strum(to_string = "Ouvert")]
    OUVERT,
    #[strum(to_string = "Fermé")]
    FERME
}

pub trait ConnecteurReference {
    fn create(&self, entity: &Reference) -> Result<()>;
    fn get_one(&self, id: &Uuid) -> Result<Reference>;
    fn get_all(&self,) -> Result<Vec<Reference>>;
    fn delete(&self, entity_id: &Uuid) -> Result<()>;
    fn update(&self, entity: &Reference) -> Result<()>;
    fn search(&self, name: Option<&String>, tags: &HashSet<Tag>, mode: ModeTags) -> Result<Vec<Reference>>;
}