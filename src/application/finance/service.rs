use log::info;
use rusqlite::{Error, Row};
use uuid::Uuid;

use crate::application::database::{self, CRUD};
use anyhow::{Context, Result};
use super::depense::Depense;



impl CRUD<Depense> for Depense {
    
    fn get_one(id: Uuid) -> Result<Depense> {
        let query = "SELECT d.id, d.libelle, d.montant 
                FROM depense as d LIMIT 1";
        database::opening_database()?
                .prepare(query)?
                .query_map([id.to_string()], map_row)?
                .next()
                .transpose()?
                .context("Not found")
    }

    fn get_all() -> Result<Vec<Depense>>{
        let query = "SELECT d.id, d.libelle, d.montant 
                FROM depense as d
                GROUP BY d.id
                ORDER BY d.libelle DESC";
        Ok(database::opening_database()?
                    .prepare(query)?
                    .query_map([], map_row)?
                    .map(|row| row.unwrap())
                    .collect::<Vec<Depense>>())
    }

    fn delete(entity: &Depense) -> Result<usize> {
        let query = "DELETE FROM depense WHERE id = ?1";
        let connexion = database::opening_database().context("Could not open database")?;
        connexion.execute(query, [String::from(entity.id)]).context("While deleting")
    }

    
    fn create(depense: &Depense) -> Result<()> {
        let id =Uuid::new_v4();
        let ref_query = "INSERT INTO depense (id, libelle, montant) VALUES (?1, ?2, ?3);";
        let connexion = database::opening_database().context("Could not open database")?;


        info!("Adding new depense: {}", depense.libelle);
        connexion.execute(ref_query, (id.to_string(), depense.libelle.clone(), depense.montant))?;
        Ok(())
    }

    fn update(depense: &Depense) -> Result<()> {
        let id =Uuid::new_v4();
        let ref_query = "UPDATE depense SET libelle=?1, montant=?2) VALUES (?1, ?2);";
        let connexion = database::opening_database().context("Could not open database")?;


        info!("Updating  depense: {}", depense.libelle);
        connexion.execute(ref_query, (id.to_string(), depense.libelle.clone(), depense.montant))?;
        Ok(())
    }


}

pub fn create_or_update(depense: &Depense) -> Result<()>  {
    match Depense::get_one(depense.id) {
        Ok(_) => Depense::update(depense),
        Err(_) => Depense::create(depense)
    }
}

fn map_row(row: &Row) -> Result<Depense, Error> {
    let id  = row.get(0)
        .and_then(|id: String| Uuid::parse_str(id.as_str()).map_err(|_| rusqlite::Error::ExecuteReturnedResults))?;
    
    Ok(Depense {
        id,
        libelle: row.get(1)?,
        montant: row.get(2)?,
    })
}

