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
    Indra(#[from] indradb::Error)
}