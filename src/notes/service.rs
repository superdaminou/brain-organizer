use std::{fs::{remove_file, File}, io::Write};


use rusqlite::{Error, Row};
use uuid::Uuid;

use crate::{database, file::construct_path};

use super::note::Note;
use anyhow::{Context, Result};

impl Note {
    
    fn create(note: &Note) -> Result<(), anyhow::Error> {
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


    fn delete(reflexion: &Note) -> Result<()> {
        reflexion.id.clone().context("Expecting id")
            .and_then(|_| database::opening_database())?
            .execute("DELETE FROM reflexion WHERE id=?1", [reflexion.id.clone()])
            .context("Failed to delete")
            .and_then(|_| remove_file(construct_path(&reflexion.filename())).context("failed to remove file"))
            .with_context(|| "An error occured")
            
    }


    fn get_all() -> Result<Vec<Note>> {
        let query = "SELECT r.id, r.sujet FROM reflexion as r";
        database::opening_database()?
                .prepare(query)?
                .query_map([], map_row)?
                .map(|row| row.context("Mapping result to Reflexion"))
                .collect::<Result<Vec<Note>>>()
    }
    
    fn update(note: &Note) -> Result<()> {
        File::options()
            .read(true)
            .write(true)
            .open(construct_path(&(&note.filename())))
            .and_then(|mut f| 
                f.write_all(note.contenu()?.as_bytes()))
            .context("Ca a explosé ")
    }
    

}

 
fn map_row(row: &Row) -> Result<Note, Error> {
    Ok(Note {
        id: row.get(0)?,
        sujet: row.get(1)?,
        contenu: String::default()
    })
}