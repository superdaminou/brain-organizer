use std::collections::HashSet;
use egui::ahash::{HashMap, HashMapExt};
use reqwest::blocking::{Body, Response};
use uuid::Uuid;
use crate::{application_error::ApplicationError, client, reference::{structs::reference::Reference, tag::Tag, ConnecteurReference, ModeTags}, server::SearchParams};

pub struct ConnecteurWebReference;

impl ConnecteurWebReference {
    pub fn new() -> ConnecteurWebReference {
        ConnecteurWebReference
    }

    fn all_tags_distinct(path: &String) -> Result<Response, ApplicationError> {
        client::get(path)
    }
}

impl ConnecteurReference for ConnecteurWebReference {
    fn create(&self, entity: &Reference) -> Result<(), ApplicationError> {
        let path = "/references".to_string();
        client::post(&path, Body::from(serde_json::to_string(entity).unwrap()));
            
        Ok(())
    }

    fn get_one(&self, id: &Uuid) -> Result<Reference, ApplicationError> {
        let path = format!("/references/{}", id);
        client::get(&path)?
            .json::<Reference>()
            .map_err(ApplicationError::from)
    }

    fn get_all(&self, ) -> Result<Vec<Reference>, ApplicationError> {
        let path = "/references".to_string();
        client::get(&path)?
            .json::<Vec<Reference>>()
            .map_err(ApplicationError::from)
    }

    fn delete(&self, entity_id: &Uuid) -> Result<(), ApplicationError> {
        let path = format!("/references/{}", entity_id);
        client::delete(&path)?
            .json::<()>()
            .map_err(ApplicationError::from)
    }

    fn update(&self, entity: &Reference) -> Result<(), ApplicationError> {
        let path = format!("/references/{}", entity.id.clone().unwrap());
    
        client::update(&path, Body::from(serde_json::to_string(entity).unwrap()))?;

        Ok(())            
    }
    
    fn search(&self, name: Option<&String>, tags: &HashSet<Tag>, mode: ModeTags) -> Result<Vec<Reference>, ApplicationError> {
        let path = "/references/search".to_string();
        let search_params = SearchParams {
            name: name.cloned(),
            tags: Some(tags.to_owned()),
            mode: Some(mode)
        };
        let mut vals = HashMap::new();
        vals.insert("name", name.cloned());
        client::post(&path, Body::from(serde_json::to_string(&search_params).unwrap()))
            .json::<Vec<Reference>>()
            .map_err(ApplicationError::from)
    }
    
    fn all_tags_distinct(&self) -> Result<Vec<Tag>, ApplicationError> {
        let path = "/tags".to_string();
        ConnecteurWebReference::all_tags_distinct(&path)?
            .json::<Vec<Tag>>()
            .map_err(ApplicationError::from)
    }
}