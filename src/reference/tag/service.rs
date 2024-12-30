use log::info;
use rusqlite::{Error, Row};

use crate::{application_error::ApplicationError, database};

use super::Tag;

pub fn get_all_distinct() -> Result<Vec<Tag>, ApplicationError>{
    info!("Getting all tags");
    let query = "SELECT DISTINCT nom FROM tag";
        database::opening_database()?
                .prepare(query)?
                .query_map([], map_row)?
                .map(|row| row.map_err(ApplicationError::from))
                .collect::<Result<Vec<Tag>, ApplicationError>>()
}

fn map_row(row: &Row) -> Result<Tag, Error> {
    row.get(0).map(Tag) 
}