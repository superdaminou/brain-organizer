use anyhow::Context;
use ilmen_http::{http::HTTPResponse, ParamsHandler, ResponseBuilder};
use uuid::Uuid;

use crate::{database::CRUD, error::ApplicationError, reference::structs::reference::Reference};


pub fn get_all(_: ParamsHandler) -> HTTPResponse {
    Reference::get_all()
        .map_err(ApplicationError::from)
        .and_then(|refs| serde_json::to_string(&refs).map_err(ApplicationError::from))
        .map(|body| ResponseBuilder::new(200, Some(body)).build())
        .unwrap_or_else(ApplicationError::into)
}

pub fn get_one(params: ParamsHandler) -> HTTPResponse {
    params.params.get("id")
        .context("Missing Params")
        .and_then(|id| Uuid::try_parse(id.as_str()).context("Cannot parse id to UUID"))
        .and_then(Reference::get_one)
        .and_then(|refs| serde_json::to_string(&refs).context("Could not serialize body"))
        .map(|body| ResponseBuilder::new(200, Some(body)).build())
        .unwrap_or_else(|err|ApplicationError::from(err).into())
        
}