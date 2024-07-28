use ilmen_http::{http::HTTPResponse, ResponseBuilder};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ApplicationError
{
    #[error("Database Error")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("An error occured")]
    DefaultError,

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Could not determine enum from: {0}")]
    EnumError(String),
    
    #[error("UnknownError")]
    Unknown,
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),

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

    #[error(transparent)]
    Indra(#[from] indradb::Error)
}

impl From<ApplicationError> for HTTPResponse {
    fn from(val: ApplicationError) -> Self {
        ResponseBuilder::new(500, Some(val.to_string())).build()
    }
}
