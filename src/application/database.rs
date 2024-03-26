use std::io::Read;

use rusqlite::{Connection, Error};

use super::{error::ApplicationError, file::opening_file};

pub fn ensuring_model(connection: Connection) -> Result<(), ApplicationError>{
    let mut init_query = String::new();  
    
     opening_file(INIT_SQL_PATH)
    .map_err(ApplicationError::from)?
    .read_to_string(&mut init_query)
    .map_err(ApplicationError::from)?;

    return connection.execute_batch(
        &init_query,
    ).map_err(ApplicationError::from);
}

pub fn opening_database() -> Result<Connection, Error> {
    println!("Try opening database file: {}", DB_PATH);
    return Connection::open(DB_PATH);
}

const INIT_SQL_PATH: &str = "init.sql";
const DB_PATH: &str = "whatsNext.db";
