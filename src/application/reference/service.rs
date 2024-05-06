use anyhow::Context;
use log::{error, info};
use rusqlite::{params_from_iter, Error, Row};
use uuid::Uuid;
use crate::application::{database, error::ApplicationError};

use super::structs::{reference::Reference, tag::Tag};

use anyhow::Result;

pub fn create(reference: &Reference) -> Result<()> {
    let id =Uuid::new_v4();
    let ref_query = "INSERT INTO reference (id, nom, url) VALUES (?1, ?2, ?3);";
    let tag_query = "INSERT INTO tag (id, nom, reference_id) VALUES (?1, ?2, ?3);";
    let connexion = database::opening_database().context("Could not open database")?;


    info!("Adding new reference: {}", reference.titre);
    connexion.execute(ref_query, (id.to_string(), reference.titre.clone(), reference.url.clone()))?;

    reference.tags.iter().map(|cat|
        connexion.execute(tag_query, (Uuid::new_v4().to_string(), cat.to_string(), id.to_string())))
            .for_each(|result|  {
                match result {
                    Err(e) => error!("Error while insertion: {}", e),
                    Ok(_) => info!("Successful")
                }
        });

    Ok(())
}

pub fn create_or_update(reference: &Reference) -> Result<()>  {
    match &reference.id {
        Some(_) => update(reference),
        None => create(reference)
    }

}


pub fn update(reference: &Reference) -> Result<()> {

    let id = Uuid::parse_str(reference.id.clone().unwrap().as_str())?;
    get_one(id)?;
    
    let ref_query = "UPDATE reference SET nom = ?1, url = ?2 WHERE id = ?3;";
    let delete_tag_query = "DELETE FROM tag WHERE reference_id = ?1;";
    let tag_query = "INSERT INTO tag (id, nom, reference_id) VALUES (?1, ?2, ?3);";
    let connexion = database::opening_database()?;


    info!("Updating  reference: {}", reference.titre);
    connexion.execute(ref_query, (reference.titre.clone(), reference.url.clone(), id.to_string()))?;

    connexion.execute(delete_tag_query, (id.to_string(),))?;

    reference.tags.iter().map(|cat|
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


pub fn delete(reference: &Reference) -> Result<usize> {
     reference.id.clone()
        .context("pas d'id")
        .and_then(|_| database::opening_database())?
        .execute("DELETE FROM tag WHERE reference_id=?1", [reference.id.clone()])?;

    reference.id.clone()
        .context("Pas d'id")
        .and_then(|_| database::opening_database())?
        .execute("DELETE FROM reference WHERE id=?1", [reference.id.clone()])
        .context("While executing delete reference")
}


pub fn get_all() -> Result<Vec<Reference>> {
    let query = "SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag FROM reference as r LEFT JOIN tag as t ON t.reference_id = r.id GROUP BY r.id;";
    Ok(database::opening_database()?
            .prepare(query)?
            .query_map([], map_row)?
            .map(|row| row.unwrap())
            .collect::<Vec<Reference>>())
}

pub fn filter_by_tags(tags: &[Tag]) -> Result<Vec<Reference>> {
    if tags.is_empty() {
        return get_all();
    }
    
    let binding = tags.iter().map(Tag::to_string).collect::<Vec<String>>();
    let tags_str  = binding.as_slice();

    let query = format!(
        "WITH ref_ids AS (SELECT reference_id FROM tag WHERE nom IN ({}))
        SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag 
        FROM reference as r 
        LEFT JOIN tag as t ON t.reference_id = r.id 
        WHERE r.id IN (SELECT * FROM ref_ids)
        GROUP BY r.id;", repeat_vars(tags.len()));

    Ok(database::opening_database()?
            .prepare(&query)?
            .query_map(params_from_iter(tags_str), map_row)?
            .map(|row| row.unwrap())
            .filter(|refe|tags.iter().all(|item| refe.tags.contains(item))) 
            .collect::<Vec<Reference>>())
            
}

fn repeat_vars(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}

pub fn get_one(id: Uuid) -> Result<Reference> {
    let query = "SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag FROM reference as r LEFT JOIN tag as t ON t.reference_id = r.id WHERE r.id = :id GROUP BY r.id LIMIT 1;";
    database::opening_database()?
            .prepare(query)?
            .query_map([id.to_string()], map_row)?
            .map(|row| row.unwrap())
            .next()
            .context("Not found")
}

fn map_row(row: &Row) -> Result<Reference, Error> {
    let tags :String = row.get(3)?;

    let categorie = tags.split(',')
        .map(Tag::try_from)
        .collect::<Result<Vec<Tag>,ApplicationError>>()
        .map_err(|_| Error::ExecuteReturnedResults)?;

    Ok(Reference {
        id: row.get(0)?,
        titre: row.get(1)?,
        url: row.get(2)?,
        tags: categorie,
    })
}