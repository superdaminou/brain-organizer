use ilmen_http::{http::HTTPResponse, RequestHandler, ResponseBuilder};

use crate::{application_error::ApplicationError, connecteur::Connecteur, reference::ConnecteurReference};



pub fn all_tags_distinct(_: &RequestHandler) -> HTTPResponse {
    Connecteur::LOCAL.all_tags_distinct()
        .map_err(ApplicationError::from)
        .and_then(|refs| serde_json::to_string(&refs).map_err(ApplicationError::from))
        .map(|body| ResponseBuilder::new(200, Some(body)).build())
        .unwrap_or_else(ApplicationError::into)
}