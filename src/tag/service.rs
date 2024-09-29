use anyhow::Context;
use log::info;
use rusqlite::{Error, Row};

use crate::database;

use super::Tag;

pub fn get_all_distinct() -> anyhow::Result<Vec<Tag>>{
    info!("Getting all tags");
    let query = "SELECT DISTINCT nom FROM tag";
        database::opening_database()?
                .prepare(query)?
                .query_map([], map_row)?
                .map(|row| row.context("Mapping result to Reflexion"))
                .collect::<anyhow::Result<Vec<Tag>>>()
}

fn map_row(row: &Row) -> Result<String, Error> {
    row.get(0) 
}