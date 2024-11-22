
use log::{info, trace};
use refinery::embed_migrations;
use anyhow::{Context, Result};
use rusqlite::Connection;
use uuid::Uuid;

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




pub trait CRUD<T> {
    fn create(entity: &T) -> Result<()>;
    fn get_one(id: &Uuid) -> Result<T>;
    fn get_all() -> Result<Vec<T>>;
    fn delete(entity: &Uuid) -> Result<usize>;
    fn update(entity: &T) -> Result<()>;

}