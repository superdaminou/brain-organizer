
use log::{info, trace};
use refinery::embed_migrations;
use rusqlite::Connection;
use crate::application_error::ApplicationError;

embed_migrations!("./src/migration");

const DB_PATH: &str = "brain_manager.db";

pub fn ensuring_model() -> Result<(), ApplicationError> {
    info!("Ensuring model and running migration if needed");
    opening_database()
    .and_then(run_migration)?;
    Ok(())
}

fn run_migration(mut connexion: Connection) -> Result<(), ApplicationError> {
    migrations::runner()
            .run(&mut connexion)?;

    Ok(())
}

pub fn opening_database() -> Result<Connection, ApplicationError> {
    trace!("Opening Database: {}", DB_PATH);
    Connection::open(DB_PATH).map_err(ApplicationError::from)
}