use std::collections::HashSet;

use chrono::NaiveDate;
use log::{debug, error, info};
use rusqlite::{Error, Row};
use uuid::Uuid;

use crate::{application_error::ApplicationError, database::{self}, reference::{structs::reference::Reference, tag::{self, Tag}, ConnecteurReference, ModeTags}};

pub struct ConnecteurDatabaseReference;

impl ConnecteurDatabaseReference {
    pub fn new() -> ConnecteurDatabaseReference {
        ConnecteurDatabaseReference
    }
}

impl ConnecteurReference for ConnecteurDatabaseReference {
    fn create(&self, reference: &Reference) -> Result<(), ApplicationError> {
        let id =Uuid::new_v4();
        let ref_query = "INSERT INTO reference (id, nom, url, date_creation, to_read) VALUES (?1, ?2, ?3, ?4, ?5);";
        let tag_query = "INSERT INTO tag (id, nom, reference_id) VALUES (?1, ?2, ?3);";
        let connexion = database::opening_database()?;


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

    fn update(&self, reference: &Reference) -> Result<(), ApplicationError> {
        let id = Uuid::parse_str(reference.id.clone().unwrap().as_str()).map_err(ApplicationError::from)?;
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


    fn delete(&self, id: &Uuid) -> Result<(), ApplicationError> {
        let reference = ConnecteurDatabaseReference::new().get_one(id)?;
        
        info!("Start deleting: {}", &reference.id.clone().unwrap_or("No Id".to_string()));
        reference.id.clone()
            .ok_or(ApplicationError::EmptyOption("id".to_string()))
            .and_then(|_| database::opening_database())?
            .execute("DELETE FROM tag WHERE reference_id=?1", [reference.id.clone()])?;

        reference.id.clone()
            .ok_or(ApplicationError::EmptyOption("id".to_string()))
            .and_then(|_| database::opening_database())?
            .execute("DELETE FROM reference WHERE id=?1", [reference.id.clone()])
            .map_err(ApplicationError::from)?;

        Ok(())
    }


    fn get_all(&self, ) -> Result<Vec<Reference>, ApplicationError> {
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


    fn get_one(&self, id: &Uuid) -> Result<Reference, ApplicationError> {
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
                .ok_or(ApplicationError::DefaultError("Expectinh ref".to_string()))
    }

    fn search(&self, name: Option<&String>, tags: &HashSet<Tag>, mode: ModeTags) -> Result<Vec<Reference>, ApplicationError> {
        let where_query = if name.is_none() || name.is_some_and(|n| n.trim().is_empty()) {""} else { &format!("AND r.nom LIKE '%{}%'", name.unwrap()) };
        let inclusive_tag_query = inclusive_query(tags, mode);
        
        let query =format!(
                "WITH results as (SELECT r.id
                FROM reference as r 
                LEFT JOIN tag as t ON t.reference_id = r.id 
                WHERE 1=1
                {}
                {} 
                GROUP BY r.id)
                SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag, r.date_creation, r.to_read from reference as r
                LEFT JOIN tag as t ON t.reference_id = r.id
                WHERE r.id IN (SELECT id from results)
                GROUP BY r.id
                ORDER BY r.date_creation DESC, tag, r.nom",inclusive_tag_query.unwrap_or_default(), where_query);
        debug!("{}", query);
        Ok(database::opening_database()?
                    .prepare(query.as_str())?
                    .query_map([], map_row)?
                    .map(|row| row.unwrap())
                    .filter(|r| is_exclusive(mode, r, tags))
                    .collect::<Vec<Reference>>())
    }
    
    fn all_tags_distinct(&self) -> Result<Vec<Tag>, ApplicationError> {
        tag::service::get_all_distinct()
    }
    
}

pub fn _search(name: &String, tags: &[Tag]) -> Result<Vec<Reference>, ApplicationError> {
    info!("Searching for : {}", name);
    let where_query = if name.trim().is_empty() {""} else { &format!("AND r.nom LIKE '%{}%'", name) };

    let mut tag_query = String::default();
    if !tags.is_empty() {
        tag_query  = format!("where r.tag = '{}';", tags.iter().map(|t|t.0.clone()).collect::<Vec<_>>().join(","))
    }

    let query =format!(
            "WITH results as (SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag, r.date_creation, r.to_read
            FROM reference as r 
            LEFT JOIN tag as t ON t.reference_id = r.id
            WHERE 1=1
            {} 
            GROUP BY r.id)

            SELECT r.id, r.nom, r.url, r.tag, r.date_creation, r.to_read from results as r
            {}
            ORDER BY r.date_creation DESC, tag, r.nom", where_query,tag_query);
    debug!("{}", query);
    Ok(database::opening_database()?
                .prepare(query.as_str())?
                .query_map([], map_row)?
                .map(|row| row.unwrap())
                .collect::<Vec<Reference>>())
}

fn is_exclusive(mode: ModeTags, reference: &Reference, tags: &HashSet<Tag>) -> bool {
    if mode == ModeTags::FERME {
        return reference.tags.is_superset(tags)
    }
    true
}

fn inclusive_query(tags: &HashSet<Tag>, mode: ModeTags) -> Option<String>
{
    let mut inclusive_tag_query = None;
    if !tags.is_empty() && mode == ModeTags::OUVERT {
        let tags_string = tags.iter().map(|t| format!("'{}'",t.0.clone()) ).collect::<Vec<_>>().join(",");
        inclusive_tag_query = format!("AND t.nom in ({})" , tags_string).into();
    }
    inclusive_tag_query
}



fn map_row(row: &Row) -> Result<Reference, Error> {
    let tags :String = row.get(3)?;

    let mut categorie = HashSet::default();
    if !tags.is_empty() {
        categorie = tags.split(',')
            .map(str::to_string)
            .map(Tag)
            .collect::<HashSet<Tag>>()
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