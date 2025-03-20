use std::{fs::{read_to_string, remove_file, File}, io::Write};
use log::info;
use rusqlite::{Error, Row};
use uuid::Uuid;
use crate::{application_error::ApplicationError, database, file::construct_path, notes::{ConnecteurNote, Note}};
pub struct ConnecteurNoteDb;

impl ConnecteurNoteDb {
    pub fn new() -> ConnecteurNoteDb {
        ConnecteurNoteDb
    }
}

impl ConnecteurNote for ConnecteurNoteDb {
    fn create(&self, note: &Note) -> Result<(), ApplicationError> {
        info!("Creating a Note");
        let id =Uuid::new_v4();
        let ref_query = "INSERT INTO reflexion (id, sujet) VALUES (?1, ?2);";
        database::opening_database().map(|connexion| connexion.execute(ref_query, (id.to_string(), note.sujet.clone())))
            .and_then(|_| File::create(construct_path(&note.filename())).map_err(ApplicationError::from))?;

        File::options()
            .read(true)
            .write(true)
            .open(construct_path(&note.filename()))
            .and_then(|mut f| 
                f.write_all(note.contenu.as_bytes()))?;

        Ok(())
    }

    fn get_one(&self, id: &String) -> Result<Note, ApplicationError> {
        info!("Getting {} From DB", id.clone());
        let query = "SELECT r.id, r.sujet FROM reflexion as r WHERE r.id =:id LIMIT 1";
        database::opening_database()?
                .prepare(query)?
                .query_map(&[(":id", &id.to_string())], map_row)?
                .map(|row| row.map_err(ApplicationError::from))
                .collect::<Result<Vec<Note>, ApplicationError>>()
                .map(|n|n.first().cloned())?
                .ok_or(ApplicationError::DefaultError("Missing note".to_string()))   
    }


    fn delete(&self, id: &String) -> Result<(), ApplicationError> {
        info!("Deleting {}", id);
        let reflexion = self.get_one(id)?;

        database::opening_database()?
            .execute("DELETE FROM reflexion WHERE id=?1", [&id.to_string()])
            .map_err(ApplicationError::from)
            .and_then(|_| remove_file(construct_path(&reflexion.filename())).map_err(ApplicationError::from))}


    fn get_all(&self, ) -> Result<Vec<Note>, ApplicationError> {
        info!("Gettin all notes");
        let query = "SELECT r.id, r.sujet FROM reflexion as r";
        database::opening_database()?
                .prepare(query)?
                .query_map([], map_row)?
                .map(|row| row.map_err(ApplicationError::from))
                .collect::<Result<Vec<Note>, ApplicationError>>()
    }
    
    fn update(&self, note: &Note) -> Result<(), ApplicationError> {
        info!("Updating note {}", note.sujet);
        File::options()
            .read(true)
            .write(true)
            .open(construct_path(&note.filename()))
            .and_then(|mut f| 
                f.write_all(note.contenu.as_bytes()))
            .map_err(|e|ApplicationError::DefaultError(e.to_string()))
    }
}
 
fn map_row(row: &Row) -> Result<Note, Error> {
    let mut note= Note {
        id: row.get(0)?,
        sujet: row.get(1)?,
        contenu:  String::default()
    };
    let path = construct_path(&note.filename());
    note.contenu = read_to_string(path).unwrap();
    Ok(note)
}