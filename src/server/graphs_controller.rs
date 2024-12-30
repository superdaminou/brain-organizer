
use ilmen_http::{http::HTTPResponse, RequestHandler, ResponseBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application_error::ApplicationError, connecteur::Connecteur, graph::{my_graph::Graph, ConnecteurGraph}};


pub fn get_all(_: &RequestHandler) -> HTTPResponse {
    Connecteur::LOCAL.get_all()
        .map_err(ApplicationError::from)
        .and_then(|refs| serde_json::to_string(&refs).map_err(ApplicationError::from))
        .map(|body| ResponseBuilder::new(200, Some(body)).build())
        .unwrap_or_else(ApplicationError::into)
}


pub fn get_one(params: &RequestHandler) -> HTTPResponse {
    params.path_params().get("id")
        .ok_or(ApplicationError::EmptyOption("id".to_string()))
        .and_then(|id|Connecteur::LOCAL.get_one(id))
        .and_then(|refs| serde_json::to_string(&refs).map_err(ApplicationError::from))
        .map(|body| ResponseBuilder::new(200, Some(body)).build())
        .unwrap_or_else(|err|err.into())
        
}

pub fn post_one(params: &RequestHandler) -> HTTPResponse {
    params.body()
        .map(|b|serde_json::from_str::<CreateGraph>(&b))
        .expect("Missing body")
        .map_err(ApplicationError::from)
        .and_then(|depense|Connecteur::LOCAL.create(&depense.into()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}

pub fn update_one(params: &RequestHandler) -> HTTPResponse {
    params.body()
        .map(|b|serde_json::from_str::<UpdateGraph>(&b))
        .expect("Missing body")
        .map_err(ApplicationError::from)
        .and_then(|depense|Connecteur::LOCAL.update(&depense.into()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}

pub fn delete(params: &RequestHandler) -> HTTPResponse {
    params.path_params().get("id")
        .ok_or(ApplicationError::EmptyOption("id".to_string()))
        .map_err(ApplicationError::from)
        .and_then(|id| Uuid::parse_str(id).map_err(ApplicationError::from))
        .and_then(|depense|Connecteur::LOCAL.delete(&depense.to_string()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}


#[derive(Serialize, Deserialize)]
struct CreateGraph {
    pub filename: String,
    pub contenu: String
}

#[derive(Serialize, Deserialize)]
struct UpdateGraph {
    pub id: Uuid,
    pub filename: String,
    pub contenu: String
}

impl From<CreateGraph> for Graph {
    fn from(value: CreateGraph) -> Self {
        Graph {
            filename: value.filename,
            contenu: value.contenu,
            ..Default::default()
        }
    }
}

impl From<UpdateGraph> for Graph {
    fn from(value: UpdateGraph) -> Self {
        Graph {
            id: value.id,
            filename: value.filename,
            contenu: value.contenu
        }
    }
}