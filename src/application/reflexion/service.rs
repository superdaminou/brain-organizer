use std::fs::{remove_file, File};

use rusqlite::{Error, Row};
use uuid::Uuid;
use crate::{application::{database, error::ApplicationError}, CsvLine};

use super::structs::Reflexion;


pub fn create(reflexion: &Reflexion) -> Result<(), ApplicationError> {
    let id =Uuid::new_v4();
    let ref_query = "INSERT INTO reflexion (id, sujet) VALUES (?1, ?2);";
    let connexion = database::opening_database().map_err(ApplicationError::from)?;

    connexion.execute(ref_query, (id.to_string(), reflexion.sujet.clone()))
    .map_err(ApplicationError::from)?;

    File::create(reflexion.get_path())?;

    Ok(())
}


pub fn delete(reflexion: &Reflexion) -> Result<(), ApplicationError> {
    reflexion.id.clone()
        .ok_or(ApplicationError::from("Pas d'id".to_string()))
        .and_then(|_| database::opening_database().map_err(ApplicationError::from))?
        .execute("DELETE FROM reflexion WHERE id=?1", [reflexion.id.clone()])
        .map_err(ApplicationError::from)
        .and_then(|_| remove_file(reflexion.get_path()).map_err(ApplicationError::from))
        
}


pub fn get_all() -> Result<Vec<Reflexion>, ApplicationError> {
    let query = "SELECT r.id, r.sujet FROM reflexion as r";
    Ok(database::opening_database()?
            .prepare(query)
            .map_err(ApplicationError::from)?
            .query_map([], map_row)?
            .map(|row| row.unwrap())
            .collect::<Vec<Reflexion>>())
}

fn map_row(row: &Row) -> Result<Reflexion, Error> {
    Ok(Reflexion {
        id: row.get(0)?,
        sujet: row.get(1)?,
    })

}

impl From<CsvLine> for Reflexion {
    fn from(value: CsvLine) -> Self {
        let split = value.split(';').map(String::from).collect::<Vec<String>>();

        Reflexion {
            id: Some(Uuid::new_v4().to_string()),
            sujet: split.get(1).expect("Missing sujet").split(',').map(String::from).collect()
        }
    }
}

impl ToString for Reflexion {
    fn to_string(&self) -> String {
        return self.sujet.to_string();
    }
}

impl Reflexion {
    pub fn to_csv(&self) -> String {
        return self.sujet.to_string() + ";";
    }
}
 

