use std::collections::HashSet;

use anyhow::Context;
use ilmen_http::{http::HTTPResponse, RequestHandler, ResponseBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application_error::ApplicationError,  reference::{client_db::ClientDatabaseReference, client_web::{ClientWebReference, ConnecteurReference}, structs::reference::Reference, tag::Tag, ModeTags}};


pub fn get_all(_: &RequestHandler) -> HTTPResponse {
    ClientDatabaseReference::get_all()
        .map_err(ApplicationError::from)
        .and_then(|refs| serde_json::to_string(&refs).map_err(ApplicationError::from))
        .map(|body| ResponseBuilder::new(200, Some(body)).build())
        .unwrap_or_else(ApplicationError::into)
}

#[derive(Serialize, Deserialize)]
pub struct SearchParams {
    pub name: Option<String>,
    pub tags: Option<HashSet<Tag>>,
    pub mode: Option<ModeTags>
}

pub fn search(search_params: &RequestHandler) -> HTTPResponse {
    search_params.body()
        .map(|body|serde_json::from_str::<SearchParams>(&body))
        .expect("Missing search params")
        .map_err(ApplicationError::from)
        .and_then(|sp| ClientDatabaseReference::search(sp.name.as_ref(), &sp.tags.unwrap_or_default(), sp.mode.unwrap_or_default()).map_err(ApplicationError::from))
        .and_then(|r| serde_json::to_string(&r).map_err(ApplicationError::from))
        .map(|response| ResponseBuilder::new(200, Some(response)).build())
        .unwrap_or_else(|e|e.into())
}

pub fn get_one(params: &RequestHandler) -> HTTPResponse {
    params.path_params().get("id")
        .context("Missing Params")
        .and_then(|id| Uuid::try_parse(id.as_str()).context("Cannot parse id to UUID"))
        .and_then(|id|ClientDatabaseReference::get_one(&id))
        .and_then(|refs| serde_json::to_string(&refs).context("Could not serialize body"))
        .map(|body| ResponseBuilder::new(200, Some(body)).build())
        .unwrap_or_else(|err|ApplicationError::from(err).into())
        
}

pub fn post_one(params: &RequestHandler) -> HTTPResponse {
    params.body()
        .map(|b|serde_json::from_str::<CreateReference>(&b))
        .expect("Missing body")
        .map_err(ApplicationError::from)
        .and_then(|reference|ClientDatabaseReference::create(&reference.into()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}

pub fn update_one(params: &RequestHandler) -> HTTPResponse {
    params.body()
        .map(|b|serde_json::from_str::<UpdateReference>(&b))
        .expect("Missing body")
        .map_err(ApplicationError::from)
        .and_then(|reference|ClientDatabaseReference::update(&reference.into()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}

pub fn delete(params: &RequestHandler) -> HTTPResponse {
    params.path_params().get("id")
        .context("Missing Params")
        .map_err(ApplicationError::from)
        .and_then(|id| Uuid::parse_str(id).map_err(|e|ApplicationError::DefaultError("Not an uuid".to_string())))
        .and_then(|reference|ClientWebReference::delete(&reference).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}


#[derive(Serialize, Deserialize)]
struct CreateReference {
    pub titre: String,
    pub url: String,
    pub tags: HashSet<Tag>,
    pub to_read: bool
}

#[derive(Serialize, Deserialize)]
struct UpdateReference {
    pub id: String,
    pub titre: String,
    pub url: String,
    pub tags: HashSet<Tag>,
    pub to_read: bool
}

impl From<CreateReference> for Reference {
    fn from(value: CreateReference) -> Self {
        Reference {
            tags: value.tags,
            titre: value.titre,
            to_read: value.to_read,
            url: value.url,
            ..Default::default()
        }
    }
}

impl From<UpdateReference> for Reference {
    fn from(value: UpdateReference) -> Self {
        Reference {
            id: Some(value.id),
            tags: value.tags,
            titre: value.titre,
            to_read: value.to_read,
            url: value.url,
            ..Default::default()
        }
    }
}