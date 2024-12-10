use std::{fs::{remove_file, File}, io::Write};
use log::info;
use rusqlite::{Error, Row};
use uuid::Uuid;
use crate::{application_error::ApplicationError, database, file::construct_path, notes::{ConnecteurNote, Note}};
use anyhow::{Context, Result};
pub struct ConnecteurNoteDb;

impl ConnecteurNoteDb {
    pub fn new() -> ConnecteurNoteDb {
        ConnecteurNoteDb
    }
}

impl ConnecteurNote for ConnecteurNoteDb {
    fn create(&self, note: &Note) -> Result<(), anyhow::Error> {
        info!("Creating a Note");
        let id =Uuid::new_v4();
        let ref_query = "INSERT INTO reflexion (id, sujet) VALUES (?1, ?2);";
        database::opening_database().map(|connexion| connexion.execute(ref_query, (id.to_string(), note.sujet.clone())))
            .and_then(|_| File::create(construct_path(&note.filename())).context("Creating file"))?;

        File::options()
            .read(true)
            .write(true)
            .open(construct_path(&(&note.filename())))
            .and_then(|mut f| 
                f.write_all(note.contenu()?.as_bytes()))?;

        Ok(())
    }

    fn get_one(&self, id: &String) -> anyhow::Result<Note> {
        info!("Getting {} From DB", id.clone());
        let query = "SELECT r.id, r.sujet FROM reflexion as r WHERE r.id =:id LIMIT 1";
        database::opening_database()?
                .prepare(query)?
                .query_map(&[(":id", &id.to_string())], map_row)?
                .map(|row| row.context("Mapping result to Reflexion"))
                .collect::<Result<Vec<Note>>>()
                .map(|n|n.first().cloned())?
                .context("Should have a note")   
    }


    fn delete(&self, id: &String) -> Result<()> {
        info!("Deleting {}", id);
        let reflexion = self.get_one(id)?;

        database::opening_database()?
            .execute("DELETE FROM reflexion WHERE id=?1", [&id.to_string()])
            .context("Failed to delete")
            .and_then(|_| remove_file(construct_path(&reflexion.filename())).context("failed to remove file"))
            .with_context(|| "An error occured")
    }


    fn get_all(&self, ) -> Result<Vec<Note>> {
        info!("Gettin all notes");
        let query = "SELECT r.id, r.sujet FROM reflexion as r";
        database::opening_database()?
                .prepare(query)?
                .query_map([], map_row)?
                .map(|row| row.context("Mapping result to Reflexion"))
                .collect::<Result<Vec<Note>>>()
    }
    
    fn update(&self, note: &Note) -> Result<(), ApplicationError> {
        info!("Updating note {}", note.sujet);
        File::options()
            .read(true)
            .write(true)
            .open(construct_path(&(&note.filename())))
            .and_then(|mut f| 
                f.write_all(note.contenu()?.as_bytes()))
            .map_err(|e|ApplicationError::DefaultError(e.to_string()))
    }
}
 
fn map_row(row: &Row) -> Result<Note, Error> {
    Ok(Note {
        id: row.get(0)?,
        sujet: row.get(1)?,
        contenu: String::default()
    })
}