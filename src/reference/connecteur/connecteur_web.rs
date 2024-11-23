use std::collections::HashSet;

use anyhow::{Context, Result};
use egui::ahash::{HashMap, HashMapExt};
use reqwest::{blocking::Body, header::HeaderMap};
use serde::de::DeserializeOwned;
use uuid::Uuid;
use crate::{reference::{structs::reference::Reference, tag::Tag, ConnecteurReference, ModeTags}, server::SearchParams};


pub struct ConnecteurWebReference;

impl ConnecteurWebReference {
    pub fn new() -> ConnecteurWebReference {
        ConnecteurWebReference
    }

    fn get<T: DeserializeOwned>(path: &String) -> Result<T> {
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
            .json::<T>()
            .context("Fuck")
    }

    fn post<T: DeserializeOwned>(path: &String, body: Body) -> Result<T> {
        let client = reqwest::blocking::Client::new();
        let mut headers= HeaderMap::new();
        headers.insert("user-agent","ILMEN/1.0".parse().unwrap());
        let user = std::env::var("USER").expect("Missing url");
        let password = std::env::var("PASSWORD").expect("Missing url");
        //let auth = BASE64_STANDARD.encode(format!("{}:{}", user, password));
        //headers.insert("Authorization", format!("Basic {}", auth).parse().unwrap());
        
        let url = std::env::var("SERVER_URL").expect("Missing url");
        client.post(format!("{}{}", url, path))
            .headers(headers.clone())
            .basic_auth(user, Some(password))
            .body(body)
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json::<T>()
            .context("Fuck")
    }
    

    fn delete<T: DeserializeOwned>(path: &String) -> Result<T> {
        let client = reqwest::blocking::Client::new();
        let mut headers= HeaderMap::new();
        headers.insert("user-agent","ILMEN/1.0".parse().unwrap());
        let user = std::env::var("USER").expect("Missing url");
        let password = std::env::var("PASSWORD").expect("Missing url");
        
        let url = std::env::var("SERVER_URL").expect("Missing url");
        client.delete(format!("{}{}", url, path))
            .headers(headers.clone())
            .basic_auth(user, Some(password))
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json::<T>()
            .context("Fuck")
    }

    fn update<T: DeserializeOwned>(path: &String, body: Body) -> Result<T> {
        let client = reqwest::blocking::Client::new();
        let mut headers= HeaderMap::new();
        headers.insert("user-agent","ILMEN/1.0".parse().unwrap());
        let user = std::env::var("USER").expect("Missing url");
        let password = std::env::var("PASSWORD").expect("Missing url");
        
        let url = std::env::var("SERVER_URL").expect("Missing url");
        client.put(format!("{}{}", url, path))
            .headers(headers.clone())
            .basic_auth(user, Some(password))
            .body(body)
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json::<T>()
            .context("Fuck")
    }

}

impl ConnecteurReference for ConnecteurWebReference {
    fn create(&self, entity: &Reference) -> Result<()> {
        let path = "/references".to_string();
        ConnecteurWebReference::post(&path, Body::from(serde_json::to_string(entity).unwrap()))
    }

    fn get_one(&self, id: &Uuid) -> Result<Reference> {
        let path = format!("/references/{}", id);
        ConnecteurWebReference::get(&path)
    }

    fn get_all(&self, ) -> Result<Vec<Reference>> {
        let path = "/references".to_string();
        ConnecteurWebReference::get(&path)
    }

    fn delete(&self, entity_id: &Uuid) -> Result<usize> {
        let path = format!("/references/{}", entity_id);
        ConnecteurWebReference::delete(&path)
    }

    fn update(&self, entity: &Reference) -> Result<()> {
        let path = format!("/references/{}", entity.id.clone().unwrap());
        ConnecteurWebReference::update(&path, Body::from(serde_json::to_string(entity).unwrap()))
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
        ConnecteurWebReference::post(&path, Body::from(serde_json::to_string(&search_params).unwrap()))
        // ConnecteurWebReference::post(&path, Body::from("{}".to_string()))
    }
}