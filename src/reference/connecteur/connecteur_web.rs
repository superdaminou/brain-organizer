use std::collections::HashSet;
use anyhow::{Context, Result};
use egui::ahash::{HashMap, HashMapExt};
use reqwest::{blocking::{Body, Response}, header::HeaderMap};
use uuid::Uuid;
use crate::{application_error::ApplicationError, client, reference::{structs::reference::Reference, tag::Tag, ConnecteurReference, ModeTags}, server::SearchParams};


pub struct ConnecteurWebReference;

impl ConnecteurWebReference {
    pub fn new() -> ConnecteurWebReference {
        ConnecteurWebReference
    }

    fn all_tags_distinct(path: &String) -> Response {
        let client = reqwest::blocking::Client::new();
        let mut headers= HeaderMap::new();
        headers.insert("user-agent","ILMEN/1.0".parse().unwrap());
        let user = std::env::var("USER").expect("Missing url");
        let password = std::env::var("PASSWORD").expect("Missing url");
        
        let url = std::env::var("SERVER_URL").expect("Missing url");
        client.get(format!("{}{}", url, path))
            .headers(headers.clone())
            .basic_auth(user, Some(password))
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
    }
}

impl ConnecteurReference for ConnecteurWebReference {
    fn create(&self, entity: &Reference) -> Result<()> {
        let path = "/references".to_string();
        client::post(&path, Body::from(serde_json::to_string(entity).unwrap()));
            
        Ok(())
    }

    fn get_one(&self, id: &Uuid) -> Result<Reference> {
        let path = format!("/references/{}", id);
        client::get(&path).
            json::<Reference>()
            .with_context(||"Error while deserializing get response".to_string())
    }

    fn get_all(&self, ) -> Result<Vec<Reference>> {
        let path = "/references".to_string();
        client::get(&path)
            .json::<Vec<Reference>>()
            .context("Error while creating reference")
    }

    fn delete(&self, entity_id: &Uuid) -> Result<()> {
        let path = format!("/references/{}", entity_id);
        client::delete(&path)
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .json::<()>()
            .context("Error while Deleting")
    }

    fn update(&self, entity: &Reference) -> Result<()> {
        let path = format!("/references/{}", entity.id.clone().unwrap());
        client::update(&path, Body::from(serde_json::to_string(entity).unwrap()))
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .json::<()>()
            .context("Error while Updating")
    }
    
    fn search(&self, name: Option<&String>, tags: &HashSet<Tag>, mode: ModeTags) -> Result<Vec<Reference>> {
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
            .context("Error while creating reference")
    }
    
    fn all_tags_distinct(&self) -> Result<Vec<Tag>> {
        let path = "/tags".to_string();
        ConnecteurWebReference::all_tags_distinct(&path)
            .json::<Vec<Tag>>()
            .context("Error while creating reference")
    }
}