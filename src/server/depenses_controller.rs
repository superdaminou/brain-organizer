
use ilmen_http::{http::HTTPResponse, RequestHandler, ResponseBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application_error::ApplicationError, connecteur::Connecteur, finance::{depense::{Depense, REPETITION}, ConnecteurDepense}};


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
        .map(|b|serde_json::from_str::<CreateDepense>(&b))
        .expect("Missing body")
        .map_err(ApplicationError::from)
        .and_then(|depense|Connecteur::LOCAL.create(&depense.into()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}

pub fn update_one(params: &RequestHandler) -> HTTPResponse {
    params.body()
        .map(|b|serde_json::from_str::<UpdateDepense>(&b))
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
        .and_then(|id| Uuid::parse_str(id).map_err(|e|ApplicationError::DefaultError("Not an uuid".to_string())))
        .and_then(|depense|Connecteur::LOCAL.delete(&depense.to_string()).map_err(ApplicationError::from))
        .map(|_| ResponseBuilder::new(200, None).build())
        .unwrap_or_else(|e| e.into())
}


#[derive(Serialize, Deserialize)]
struct CreateDepense {
    pub libelle: String,
    pub montant: f32,
    pub repetition: REPETITION
}

#[derive(Serialize, Deserialize)]
struct UpdateDepense {
    pub id: Uuid,
    pub libelle: String,
    pub montant: f32,
    pub repetition: REPETITION
}

impl From<CreateDepense> for Depense {
    fn from(value: CreateDepense) -> Self {
        Depense {
            libelle: value.libelle,
            montant: value.montant,
            repetition: value.repetition,
            ..Default::default()
        }
    }
}

impl From<UpdateDepense> for Depense {
    fn from(value: UpdateDepense) -> Self {
        Depense {
            id: Some(value.id),
            libelle: value.libelle,
            montant: value.montant,
            repetition: value.repetition,
            ..Default::default()
        }
    }
}