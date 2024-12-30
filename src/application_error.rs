use chrono::ParseError;
use ilmen_http::{http::HTTPResponse, ResponseBuilder};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ApplicationError
{
    #[error(transparent)]
    DatabaseError(#[from] rusqlite::Error),

    #[error("An error occured: {0}")]
    DefaultError(String),

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Could not determine enum from: {0}")]
    EnumError(String),
    
    #[error("UnknownError")]
    Unknown,

    #[error("I was expecting a: {0}")]
    EmptyOption(String),

    #[error(transparent)]
    RefineryError(#[from] refinery::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    HttpClientError(#[from] reqwest::Error),

    #[error(transparent)]
    DotParserError(#[from] ilmen_dot_parser::ParsingError),

    #[error(transparent)]
    ParseDateError(#[from] ParseError),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),


    #[error("failed to read the key file")]
    FileReadError(#[source] std::io::Error),

    #[error("failed to read the key file")]
    FileWriteError(#[source] std::io::Error),
    
    #[error("failed to delete the key file")]
    FileDeleteError(#[source] std::io::Error),

    #[error(transparent)]
    EframeError(#[from] eframe::Error),
    
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

impl From<ApplicationError> for HTTPResponse {
    fn from(val: ApplicationError) -> Self {
        ResponseBuilder::new(500, Some(val.to_string())).build()
    }
}