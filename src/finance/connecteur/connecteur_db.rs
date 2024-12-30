use log::info;
use rusqlite::{Error, Row};
use uuid::Uuid;
use crate::{application_error::ApplicationError, database, finance::{depense::{Depense, REPETITION}, ConnecteurDepense}};
pub struct ConnecteurDepenseDb;

impl ConnecteurDepenseDb {
    pub fn new() -> ConnecteurDepenseDb {
        ConnecteurDepenseDb
    }
}

impl ConnecteurDepense for ConnecteurDepenseDb {
    fn get_one(&self, id: &String) -> Result<Depense, ApplicationError> {
        let query = "SELECT d.id, d.libelle, d.montant, d.repetition 
                FROM depense as d LIMIT 1";
        database::opening_database()?
                .prepare(query)?
                .query_map([id.to_string()], map_row)?
                .next()
                .transpose()
                .map_err(ApplicationError::from)?
                .ok_or_else(||ApplicationError::DefaultError("Expecting a depense".to_string()))
    }

    fn get_all(&self) -> Result<Vec<Depense>, ApplicationError>{
        let query = "SELECT d.id, d.libelle, d.montant, d.repetition 
                FROM depense as d
                GROUP BY d.id
                ORDER BY d.libelle DESC";
        Ok(database::opening_database()?
                    .prepare(query)?
                    .query_map([], map_row)?
                    .map(|row| row.unwrap())
                    .collect::<Vec<Depense>>())
    }

    fn delete(&self, entity: &String) -> Result<(), ApplicationError> {
        let query = "DELETE FROM depense WHERE id = ?1";
        let connexion = database::opening_database().map_err(ApplicationError::from)?;
        connexion.execute(query, [entity.to_string()]).map_err(ApplicationError::from)?;

        Ok(())
    }

    
    fn create(&self, depense: &Depense) -> Result<(), ApplicationError> {
        let id =Uuid::new_v4();
        let ref_query = "INSERT INTO depense (id, libelle, montant, repetition) VALUES (?1, ?2, ?3, ?4);";
        let connexion = database::opening_database().map_err(ApplicationError::from)?;


        info!("Adding new depense: {}", depense.libelle);
        connexion.execute(ref_query, (id.to_string(), depense.libelle.clone(), depense.montant, depense.repetition.to_string()))?;
        Ok(())
    }

    fn update(&self, depense: &Depense) -> Result<(), ApplicationError> {
        let ref_query = "UPDATE depense SET libelle=?1, montant=?2, repetition=?3 WHERE id = ?4;";
        let connexion = database::opening_database().map_err(ApplicationError::from)?;


        info!("Updating  depense: {}", depense.libelle);
        connexion.execute(ref_query, (depense.libelle.clone(), depense.montant, depense.repetition.code(), depense.id.unwrap().to_string()))?;
        Ok(())
    }

}


fn map_row(row: &Row) -> Result<Depense, Error> {
    let id  = row.get(0)
        .and_then(|id: String| Uuid::parse_str(id.as_str()).map_err(|_| rusqlite::Error::ExecuteReturnedResults))?;
    
    let repetition: String=  row.get(3)?;

    Ok(Depense {
        id: Some(id),
        libelle: row.get(1)?,
        montant: row.get(2)?,
        repetition: REPETITION::try_from(repetition.as_str()).unwrap(),
    })
}