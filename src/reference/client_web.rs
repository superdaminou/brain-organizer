use std::{collections::HashSet, f64::consts::E, fmt::format};

use anyhow::{Context, Result};
use egui::ahash::{HashMap, HashMapExt};
use log::info;
use reqwest::{blocking::Body, header::HeaderMap};
use serde::de::DeserializeOwned;
use uuid::Uuid;
use crate::server::SearchParams;

use super::{structs::reference::Reference, tag::Tag, ModeTags};


pub struct ClientWebReference;

impl ClientWebReference {
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

impl ConnecteurReference for ClientWebReference {
    fn create(entity: &Reference) -> Result<()> {
        let path = format!("/references");
        ClientWebReference::post(&path, Body::from(serde_json::to_string(entity).unwrap()))
    }

    fn get_one(id: &Uuid) -> Result<Reference> {
        let path = format!("/references/{}", id);
        ClientWebReference::get(&path)
    }

    fn get_all() -> Result<Vec<Reference>> {
        let path = format!("/references");
        ClientWebReference::get(&path)
    }

    fn delete(entity_id: &Uuid) -> Result<usize> {
        let path = format!("/references/{}", entity_id);
        ClientWebReference::delete(&path)
    }

    fn update(entity: &Reference) -> Result<()> {
        let path = format!("/references/{}", entity.id.clone().unwrap());
        ClientWebReference::update(&path, Body::from(serde_json::to_string(entity).unwrap()))
    }
    
    fn search(name: Option<&String>, tags: &HashSet<Tag>, mode: ModeTags) -> Result<Vec<Reference>> {
        let path = format!("/references/search");
        let search_params = SearchParams {
            name: name.cloned(),
            tags: Some(tags.to_owned()),
            mode: Some(mode)
        };
        let mut vals = HashMap::new();
        vals.insert("name", name.cloned());
        //ClientWebReference::post(&path, Body::from(serde_json::to_string(&vals).unwrap()))
        ClientWebReference::post(&path, Body::from("{}".to_string()))
    }
}


pub trait ConnecteurReference {
    fn create(entity: &Reference) -> Result<()>;
    fn get_one(id: &Uuid) -> Result<Reference>;
    fn get_all() -> Result<Vec<Reference>>;
    fn delete(entity_id: &Uuid) -> Result<usize>;
    fn update(entity: &Reference) -> Result<()>;
    fn search(name: Option<&String>, tags: &HashSet<Tag>, mode: ModeTags) -> Result<Vec<Reference>>;
}