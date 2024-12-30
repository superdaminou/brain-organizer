use ilmen_http::{http::HTTPResponse, RequestHandler, ResponseBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application_error::ApplicationError, connecteur::Connecteur, notes::{ConnecteurNote, Note}};

pub fn get_all(_: &RequestHandler) -> HTTPResponse {
    Connecteur::LOCAL.get_all()
        .map_err(ApplicationError::from)
        .and_then(|note| serde_json::to_string(&note).map_err(ApplicationError::from))
        .map(|body| ResponseBuilder::new(200, Some(body)).build())
        .unwrap_or_else(ApplicationError::into)
}


pub fn get_one(params: &RequestHandler) -> HTTPResponse {
    params.path_params().get("id")
        .ok_or(ApplicationError::EmptyOption("id".to_string()))
        .and_then(|id| Uuid::try_parse(id.as_str()).map_err(ApplicationError::from))
        .and_then(|id|Connecteur::LOCAL.get_one(&id.to_string()))
        .and_then(|refs| serde_json::to_string(&refs).map_err(ApplicationError::from))
        .map(|body| ResponseBuilder::new(200, Some(body)).build())
        .unwrap_or_else(|err|err.into())
        
}

pub fn post_one(params: &RequestHandler) -> HTTPResponse {
    params.body()
        .map(|b|serde_json::from_str::<CreateNote>(&b))
        .expect("Missing body")
        .map_err(ApplicationError::from)
        .and_then(|reference|Connecteur::LOCAL.create(&reference.into()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}

pub fn update_one(params: &RequestHandler) -> HTTPResponse {
    params.body()
        .map(|b|serde_json::from_str::<UpdateNote>(&b))
        .expect("Missing body")
        .map_err(ApplicationError::from)
        .and_then(|note|Connecteur::LOCAL.update(&note.into()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}

pub fn delete(params: &RequestHandler) -> HTTPResponse {
    params.path_params().get("id")
        .ok_or(ApplicationError::EmptyOption("id".to_string()))
        .map_err(ApplicationError::from)
        .and_then(|id| Uuid::parse_str(id).map_err(ApplicationError::from))
        .and_then(|note|Connecteur::LOCAL.delete(&note.to_string()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}


#[derive(Serialize, Deserialize)]
struct CreateNote {
    pub sujet: String,
    pub contenu: String
}

#[derive(Serialize, Deserialize)]
struct UpdateNote {
    pub id: String,
    pub sujet: String,
    pub contenu: String
}

impl From<CreateNote> for Note {
    fn from(value: CreateNote) -> Self {
        Note {
            contenu: value.contenu,
            id: Uuid::new_v4().to_string(),
            sujet: value.sujet
        }
    }
}

impl From<UpdateNote> for Note {
    fn from(value: UpdateNote) -> Self {
        Note {
            contenu: value.contenu,
            id: value.id,
            sujet: value.sujet
        }
    }
}