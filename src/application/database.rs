
use log::{info, trace};
use rusqlite::Connection;
use refinery::embed_migrations;
use anyhow::{Context, Result};

embed_migrations!("./src/migration");

const DB_PATH: &str = "brain_manager.db";

pub fn ensuring_model() -> Result<(), anyhow::Error> {
    info!("Ensuring model and running migration if needed");
    opening_database()
        .and_then(|mut connexion | 
            migrations::runner()
            .run(&mut connexion)
            .with_context(||"Could not run migration"))?;
    Ok(())
}

pub fn opening_database() -> Result<Connection> {
    trace!("Opening Database: {}", DB_PATH);
    Connection::open(DB_PATH).with_context(||"Couldn't open database")
}

