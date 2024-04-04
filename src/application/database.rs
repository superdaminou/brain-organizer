
use rusqlite::Connection;
use refinery::embed_migrations;

embed_migrations!("./src/migration");

use super::error::ApplicationError;

pub fn ensuring_model() {
    let mut connexion = Connection::open(DB_PATH).unwrap();
    migrations::runner().run(&mut connexion).unwrap();
}

pub fn opening_database() -> Result<Connection, ApplicationError> {
    println!("Try opening database file: {}", DB_PATH);
    Connection::open(DB_PATH).map_err(ApplicationError::from)
}

const DB_PATH: &str = "whatsNext.db";
