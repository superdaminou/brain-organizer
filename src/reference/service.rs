use std::collections::{BTreeSet, HashSet};

use anyhow::Context;
use chrono::NaiveDate;
use log::{debug, error, info};
use rusqlite::{Error, Row};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{database::{self, CRUD}, error::ApplicationError, tag::Tag};

use super::structs::{reference::Reference};

use anyhow::Result;

impl CRUD<Reference> for Reference {
    fn create(reference: &Reference) -> Result<()> {
        let id =Uuid::new_v4();
        let ref_query = "INSERT INTO reference (id, nom, url, date_creation, to_read) VALUES (?1, ?2, ?3, ?4, ?5);";
        let tag_query = "INSERT INTO tag (id, nom, reference_id) VALUES (?1, ?2, ?3);";
        let connexion = database::opening_database().context("Could not open database")?;


        info!("Adding new reference: {}", reference.titre);
        connexion.execute(ref_query, (id.to_string(), reference.titre.clone(), reference.url.clone(), reference.date_creation.to_string(), reference.to_read))?;

        reference.tags.iter().map(|tag|
            connexion.execute(tag_query, (Uuid::new_v4().to_string(), tag.0.clone(), id.to_string())))
                .for_each(|result|  {
                    match result {
                        Err(e) => error!("Error while insertion: {}", e),
                        Ok(_) => info!("Successful")
                    }
            });

        Ok(())
    }



    fn update(reference: &Reference) -> Result<()> {

        let id = Uuid::parse_str(reference.id.clone().unwrap().as_str())?;
        Self::get_one(id)?;
        
        let ref_query = "UPDATE reference SET nom = ?1, url = ?2 WHERE id = ?3;";
        let delete_tag_query = "DELETE FROM tag WHERE reference_id = ?1;";
        let tag_query = "INSERT INTO tag (id, nom, reference_id) VALUES (?1, ?2, ?3);";
        let connexion = database::opening_database()?;


        info!("Updating  reference: {}", reference.titre);
        connexion.execute(ref_query, (reference.titre.clone(), reference.url.clone(), id.to_string()))?;

        connexion.execute(delete_tag_query, (id.to_string(),))?;

        reference.tags.iter().map(|tag|
            connexion.execute(tag_query, (Uuid::new_v4().to_string(), tag.0.to_string(), id.to_string()))
                .map_err(ApplicationError::from))
                .for_each(|result|  {
                    match result {
                        Err(e) => error!("Error while insertion: {}", e),
                        Ok(_) => info!("Successful")
                    }
            });

        Ok(())
    }


    fn delete(reference: &Reference) -> Result<usize> {
        info!("Start deleting: {}", &reference.id.clone().unwrap_or("No Id".to_string()));
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


    fn get_all() -> Result<Vec<Reference>> {
        let query = "SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag, r.date_creation, r.to_read
            FROM reference as r 
            LEFT JOIN tag as t ON t.reference_id = r.id 
            GROUP BY r.id
            ORDER BY r.date_creation DESC, tag, r.nom";
        Ok(database::opening_database()?
                    .prepare(query)?
                    .query_map([], map_row)?
                    .map(|row| row.unwrap())
                    .collect::<Vec<Reference>>())
    }


    fn get_one(id: Uuid) -> Result<Reference> {
        let query = "SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag, r.date_creation, r.to_read
            FROM reference as r 
            LEFT JOIN tag as t ON t.reference_id = r.id 
            WHERE r.id = :id 
            GROUP BY r.id 
            LIMIT 1;";
        database::opening_database()?
                .prepare(query)?
                .query_map([id.to_string()], map_row)?
                .next()
                .transpose()?
                .context("Not found")
    }
}

pub fn create_or_update(reference: &Reference) -> Result<()>  {
    match &reference.id {
        Some(_) => Reference::update(reference),
        None => Reference::create(reference)
    }
}

pub fn search(name: &String, tags: &[Tag]) -> Result<Vec<Reference>> {
    info!("Searching for : {}", name);
    let where_query = if name.trim().is_empty() { ""} else { &format!("AND r.nom LIKE '%{}%'", name) };
    let mut tag_query = String::default();
    if !tags.is_empty() {
        tag_query  = "AND t.nom in ('".to_string() + &tags.iter()
            .map(|t|t.0.clone())
            .reduce(|acc, e| acc + "','" + &e)
            .unwrap_or_default() + "')";
    }

    let query =format!(
            "SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag, r.date_creation, r.to_read
            FROM reference as r 
            LEFT JOIN tag as t ON t.reference_id = r.id
            WHERE 1=1
            {} 
            {}
            GROUP BY r.id
            ORDER BY r.date_creation DESC, tag, r.nom", where_query, tag_query);
    debug!("{}", query);
    Ok(database::opening_database()?
                .prepare(query.as_str())?
                .query_map([], map_row)?
                .map(|row| row.unwrap())
                //.filter(|refe| refe.tags.iter().any(|t| tags.is_empty() || tags.contains(t)))
                .collect::<Vec<Reference>>())
}


fn map_row(row: &Row) -> Result<Reference, Error> {
    let tags :String = row.get(3)?;

    let mut categorie = BTreeSet::default();
    if !tags.is_empty() {
        categorie = tags.split(',')
            .map(str::to_string)
            .map(|t| Tag(t))
            .collect::<BTreeSet<Tag>>()
    }

    let date: String = row.get(4)?;
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
    .map_err(|e|{
        error!("{}",e.to_string());
        rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, date.into())
    })?;

    Ok(Reference {
        id: row.get(0)?,
        titre: row.get(1)?,
        url: row.get(2)?,
        tags: categorie,
        date_creation:  date,
        to_read: row.get(5)?
    })
}