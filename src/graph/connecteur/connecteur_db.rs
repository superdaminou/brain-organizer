use std::{fs::{read_to_string, File}, io::Write};
use log::info;
use rusqlite::{Error, Row};
use uuid::Uuid;
use crate::{application_error::ApplicationError, database, file::construct_path, graph::{my_graph::Graph, ConnecteurGraph}};
pub struct ConnecteurGraphDb;

impl ConnecteurGraphDb {
    pub fn new() -> ConnecteurGraphDb {
        ConnecteurGraphDb
    }
}

impl ConnecteurGraph for ConnecteurGraphDb {

    fn create(&self, my_graph: &Graph) -> Result<(), ApplicationError> {
        let id =Uuid::new_v4();
        let query = "INSERT INTO graph (id, filename) VALUES (?1, ?2);";
        let connexion = database::opening_database()?;


        info!("Adding new graph: {}", my_graph.filename());
        connexion.execute(query, (id.to_string(), my_graph.filename()))?;

        File::create(construct_path(&(my_graph.filename()))).map_err(ApplicationError::from)?;
        Ok(())
    }


    fn update(&self, graph: &Graph) -> Result<(), ApplicationError> {

        let id = graph.id;
        self.get_one(&id.to_string())?;
        info!("Updating  graph: {} with content {}", graph.filename, graph.contenu);

        File::options()
            .read(true)
            .write(true)
            .open(construct_path(&graph.filename))
            .and_then(|mut f| 
                f.write_all(graph.contenu.as_bytes()))?;

        Ok(())
    }


    fn delete(&self, graph: &String) -> Result<(), ApplicationError> {
        info!("Start deleting: {}", &graph);
        database::opening_database()?
            .execute("DELETE FROM graph WHERE id=?1", [graph.to_string()])
            .map_err(ApplicationError::from)?;

        Ok(())
    }


    fn get_all(&self) -> Result<Vec<Graph>, ApplicationError> {
        let query = "SELECT g.id, g.filename
            FROM graph as g ;";
        Ok(database::opening_database()?
                    .prepare(query)?
                    .query_map([], map_row)?
                    .map(|row| row.unwrap())
                    .collect::<Vec<Graph>>())
    }


    fn get_one(&self, id: &String) -> Result<Graph, ApplicationError> {
        let query = "SELECT g.id, g.filename
            FROM graph as g 
            WHERE g.id = :id 
            LIMIT 1;";
        database::opening_database()?
                .prepare(query)?
                .query_map([id.to_string()], map_row)?
                .next()
                .transpose()?
                .ok_or(ApplicationError::EmptyOption(format!("Graph for {}", id)))
    }

}
 
fn map_row(row: &Row) -> Result<Graph, Error> {
    let id  = row.get(0)
        .and_then(|id: String| Uuid::parse_str(id.as_str()).map_err(|_| rusqlite::Error::ExecuteReturnedResults))?;

    let filename : String = row.get(1)?;
    let contenu = read_to_string(construct_path(&filename)).unwrap();
    
    Ok(Graph {
        id,
        filename,
        contenu
    })
}