use std::fs::File;

use log::info;
use rusqlite::{ Error, Row};
use uuid::Uuid;
use crate::application::dot_parser::dot_graph::DotGraph;
use crate::application::file::construct_path;
use crate::application::{database::{self, CRUD}, dot_parser::dot_parser::graph_from_file, error::ApplicationError};
use anyhow::{Context, Result};

#[derive(PartialEq, Eq, Clone)]
pub struct  Graph {
    pub id: Uuid,
    pub filename: String,
}


impl Default for Graph {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            filename: String::default(),
        }
    }
}

impl From<&String> for Graph {
    fn from(value: &String) -> Self {
        Self {
            filename: value.clone(),
            id: Uuid::new_v4()
        }
    }
}


impl CRUD<Graph> for Graph {
    fn create(my_graph: &Graph) -> Result<()> {
        let id =Uuid::new_v4();
        let query = "INSERT INTO graph (id, filename) VALUES (?1, ?2);";
        let connexion = database::opening_database().context("Could not open database")?;


        info!("Adding new graph: {}", my_graph.filename);
        connexion.execute(query, (id.to_string(), my_graph.filename.clone()))?;

        File::create(construct_path(&(my_graph.filename.to_string() +".dot"))).context("Creating file")?;
        Ok(())
    }


    fn update(graph: &Graph) -> Result<()> {

        let id = graph.id;
        Self::get_one(id)?;
        
        let ref_query = "UPDATE reference SET nom = ?1 WHERE id = ?3;";
        let connexion = database::opening_database()?;


        info!("Updating  graph: {}", graph.filename);
        connexion.execute(ref_query, (graph.filename.clone(), id.to_string()))?;
        Ok(())
    }


    fn delete(graph: &Graph) -> Result<usize> {
        info!("Start deleting: {}", &graph.filename);
        database::opening_database()?
            .execute("DELETE FROM graph WHERE id=?1", [graph.id.to_string()])
            .context("While executing delete graph")
    }


    fn get_all() -> Result<Vec<Graph>> {
        let query = "SELECT g.id, g.filename
            FROM graph as g ;";
        Ok(database::opening_database()?
                    .prepare(query)?
                    .query_map([], map_row)?
                    .map(|row| row.unwrap())
                    .collect::<Vec<Graph>>())
    }


    fn get_one(id: Uuid) -> Result<Graph> {
        let query = "SELECT g.id, g.filename
            FROM graph as g 
            WHERE g.id = :id 
            LIMIT 1;";
        database::opening_database()?
                .prepare(query)?
                .query_map([id.to_string()], map_row)?
                .next()
                .transpose()?
                .context("Not found")
    }

}

fn map_row(row: &Row) -> Result<Graph, Error> {
    let id  = row.get(0)
        .and_then(|id: String| Uuid::parse_str(id.as_str()).map_err(|_| rusqlite::Error::ExecuteReturnedResults))?;
    
    Ok(Graph {
        id,
        filename: row.get(1)?,
        ..Default::default()
    })
}



impl Graph {
    
    pub  fn load_graph(&self) -> Result<DotGraph, ApplicationError> {
        graph_from_file(&construct_path(&self.filename))
     }
}


 
