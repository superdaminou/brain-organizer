
use log::info;
use rusqlite::Connection;
use refinery::embed_migrations;
use super::error::ApplicationError;

embed_migrations!("./src/migration");

const DB_PATH: &str = "whatsNext.db";

pub fn ensuring_model() -> Result<(), ApplicationError> {
    info!("Ensuring model and running migration if needed");
    Connection::open(DB_PATH).map_err(ApplicationError::from)
        .and_then(|mut connexion | 
            migrations::runner().run(&mut connexion)
            .map_err(ApplicationError::from))?;

    Ok(())
}

pub fn opening_database() -> Result<Connection, ApplicationError> {
    info!("Ensuring Database: {}", DB_PATH);
    Connection::open(DB_PATH).map_err(ApplicationError::from)
}

