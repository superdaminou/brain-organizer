use log::{error, info};
use rusqlite::{Error, Row};
use uuid::Uuid;
use crate::application::{database, error::ApplicationError};

use super::structs::{Reference, Tag};


pub fn create(reference: &Reference) -> Result<(), ApplicationError> {
    let id =Uuid::new_v4();
    let ref_query = "INSERT INTO reference (id, nom, url) VALUES (?1, ?2, ?3);";
    let tag_query = "INSERT INTO tag (id, nom, reference_id) VALUES (?1, ?2, ?3);";
    let connexion = database::opening_database().map_err(ApplicationError::from)?;


    info!("Adding new reference: {}", reference.titre);
    connexion.execute(ref_query, (id.to_string(), reference.titre.clone(), reference.url.clone()))
    .map_err(ApplicationError::from)?;

    reference.categorie.iter().map(|cat|
        connexion.execute(tag_query, (Uuid::new_v4().to_string(), cat.to_string(), id.to_string()))
            .map_err(ApplicationError::from))
            .for_each(|result|  {
                match result {
                    Err(e) => error!("Error while insertion: {}", e),
                    Ok(_) => info!("Successful")
                }
        });

    Ok(())
}

pub fn create_or_update(reference: &Reference) -> Result<(), ApplicationError>  {
    match &reference.id {
        Some(_) => update(reference),
        None => create(reference)
    }

}


pub fn update(reference: &Reference) -> Result<(), ApplicationError> {

    let id = Uuid::parse_str(reference.id.clone().unwrap().as_str()).map_err(ApplicationError::from)?;
    get_one(id)?;
    
    let ref_query = "UPDATE reference SET nom = ?1, url = ?2 WHERE id = ?3;";
    let delete_tag_query = "DELETE FROM tag WHERE reference_id = ?1;";
    let tag_query = "INSERT INTO tag (id, nom, reference_id) VALUES (?1, ?2, ?3);";
    let connexion = database::opening_database().map_err(ApplicationError::from)?;


    info!("Updating  reference: {}", reference.titre);
    connexion.execute(ref_query, (reference.titre.clone(), reference.url.clone(), id.to_string()))
    .map_err(ApplicationError::from)?;

    connexion.execute(delete_tag_query, (id.to_string(),))
    .map_err(ApplicationError::from)?;

    reference.categorie.iter().map(|cat|
        connexion.execute(tag_query, (Uuid::new_v4().to_string(), cat.to_string(), id.to_string()))
            .map_err(ApplicationError::from))
            .for_each(|result|  {
                match result {
                    Err(e) => error!("Error while insertion: {}", e),
                    Ok(_) => info!("Successful")
                }
        });

    Ok(())
}


pub fn delete(reference: &Reference) -> Result<usize, ApplicationError> {
     reference.id.clone()
        .ok_or(ApplicationError::from("Pas d'id".to_string()))
        .and_then(|_| database::opening_database().map_err(ApplicationError::from))?
        .execute("DELETE FROM tag WHERE reference_id=?1", [reference.id.clone()])
        .map_err(ApplicationError::from)?;

    reference.id.clone()
        .ok_or(ApplicationError::from("Pas d'id".to_string()))
        .and_then(|_| database::opening_database().map_err(ApplicationError::from))?
        .execute("DELETE FROM reference WHERE id=?1", [reference.id.clone()])
        .map_err(ApplicationError::from)
        
}


pub fn get_all() -> Result<Vec<Reference>, ApplicationError> {
    let query = "SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag FROM reference as r LEFT JOIN tag as t ON t.reference_id = r.id GROUP BY r.id;";
    Ok(database::opening_database()?
            .prepare(query)
            .map_err(ApplicationError::from)?
            .query_map([], map_row)?
            .map(|row| row.unwrap())
            .collect::<Vec<Reference>>())
}

pub fn get_one(id: Uuid) -> Result<Option<Reference>, ApplicationError> {
    let query = "SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag FROM reference as r LEFT JOIN tag as t ON t.reference_id = r.id WHERE r.id = :id GROUP BY r.id LIMIT 1;";
    Ok(database::opening_database()?
            .prepare(query)
            .map_err(ApplicationError::from)?
            .query_map([id.to_string()], map_row)?
            .map(|row| row.unwrap()).next())
}

fn map_row(row: &Row) -> Result<Reference, Error> {
    let tags :String = row.get(3)?;

    Ok(Reference {
        id: row.get(0)?,
        titre: row.get(1)?,
        url: row.get(2)?,
        categorie: tags.split(',').map(String::from).map(Tag::from).collect::<Vec<Tag>>(),
    })

}