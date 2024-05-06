use std::fs::{remove_file, File};


use rusqlite::{Error, Row};
use uuid::Uuid;
use crate::application::{database, file::construct_path};

use super::structs::Reflexion;
use anyhow::{Context, Result};


pub fn create(reflexion: &Reflexion) -> Result<(), anyhow::Error> {
    let id =Uuid::new_v4();
    let ref_query = "INSERT INTO reflexion (id, sujet) VALUES (?1, ?2);";
    let connexion = database::opening_database()?;

    connexion.execute(ref_query, (id.to_string(), reflexion.sujet.clone()))?;

    File::create(construct_path(reflexion))?;

    Ok(())
}


pub fn delete(reflexion: &Reflexion) -> Result<()> {
    reflexion.id.clone().context("Expecting id")
        .and_then(|_| database::opening_database())?
        .execute("DELETE FROM reflexion WHERE id=?1", [reflexion.id.clone()])
        .context("Failed to delete")
        .and_then(|_| remove_file(construct_path(reflexion)).context("failed to remove file"))
        .with_context(|| "An error occured")
        
}


pub fn get_all() -> Result<Vec<Reflexion>> {
    let query = "SELECT r.id, r.sujet FROM reflexion as r";
    database::opening_database()?
            .prepare(query)?
            .query_map([], map_row)?
            .map(|row| row.context("While mapping"))
            .collect::<Result<Vec<Reflexion>>>()
}

fn map_row(row: &Row) -> Result<Reflexion, Error> {
    Ok(Reflexion {
        id: row.get(0)?,
        sujet: row.get(1)?,
    })

}
 

