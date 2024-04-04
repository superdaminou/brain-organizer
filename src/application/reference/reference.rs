use rusqlite::{Error, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{application::{database, error::ApplicationError}, CsvLine};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    pub id: Option<String>,
    pub titre: String,
    pub url: String,
    pub categorie: Vec<Tag>
}

pub type Tag = String;

pub fn create(contenu: &Reference) -> Result<(), ApplicationError> {
    let id =Uuid::new_v4();
    let ref_query = "INSERT INTO reference (id, nom, url) VALUES (?1, ?2, ?3);";
    let tag_query = "INSERT INTO tag (id, nom, reference_id) VALUES (?1, ?2, ?3);";
    let connexion = database::opening_database().map_err(ApplicationError::from)?;


    connexion.execute(ref_query, (id.to_string(), contenu.titre.clone(), contenu.url.clone()))
    .map_err(ApplicationError::from)?;

    contenu.categorie.iter().map(|cat|
        connexion.execute(tag_query, (Uuid::new_v4().to_string(), cat, id.to_string()))
            .map_err(ApplicationError::from))
            .for_each(|result|  {
                match result {
                    Err(e) => println!("Error while insertion: {}", e),
                    Ok(_) => println!("Successful")
                }
        });

    return Ok(());
}


pub fn delete(reference: &Reference) -> Result<usize, ApplicationError> {
     return reference.id.clone()
        .ok_or(ApplicationError::from("Pas d'id".to_string()))
        .and_then(|_| database::opening_database().map_err(ApplicationError::from))?
        .execute("DELETE FROM reference WHERE id=?1", [reference.id.clone()])
            .map_err(ApplicationError::from);
}


pub fn get_all() -> Result<Vec<Reference>, ApplicationError> {
    let query = "SELECT r.id, r.nom, r.url, coalesce(GROUP_CONCAT(t.nom), '') as tag FROM reference as r LEFT JOIN tag as t ON t.reference_id = r.id GROUP BY r.id;";
    return Ok(database::opening_database()?
            .prepare(query)
            .map_err(ApplicationError::from)?
            .query_map([], |row| map_row(row))?
            .map(|row| row.unwrap())
            .collect::<Vec<Reference>>());
}

fn map_row(row: &Row) -> Result<Reference, Error> {
    let tags :String = row.get(3)?;

    Ok(Reference {
        id: row.get(0)?,
        titre: row.get(1)?,
        url: row.get(2)?,
        categorie: tags.split(",").map(String::from).collect::<Vec<String>>(),
    })

}


impl From<CsvLine> for Reference {
    fn from(value: CsvLine) -> Self {
        let mut split = value.split(";").map(String::from).collect::<Vec<String>>();

        Reference {
            id: Some(Uuid::new_v4().to_string()),
            titre: split.get(0).expect("Missing title").to_string(),
            categorie: split.get(1).expect("Missing tag").split(",").map(String::from).collect(),
            url: split.get(2).expect("Missing url").to_string()
        }
    }
}
 

