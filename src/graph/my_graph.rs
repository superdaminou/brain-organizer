use std::{fs::{read_to_string, File}, io::{self}};

use ilmen_dot_parser::DotGraph;
use log::info;
use rusqlite::{ Error, Row};
use uuid::Uuid;

use crate::{application_error::ApplicationError, connecteur::Connecteur, database::{self, CRUD}, file::construct_path, gui::{EditableFile, Fileable}};

#[derive(PartialEq, Eq, Clone)]
pub struct  Graph {
    pub id: Uuid,
    filename: String,
}

impl Fileable for Graph {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn filename(&self) -> String {
        self.filename.clone()
    }

    fn contenu(&self, connecteur: &Connecteur) -> String {
        self.contenu().unwrap_or("Failed".to_string())
    }

    fn write(file: &EditableFile, connecteur: &Connecteur) -> Result<(), ApplicationError> {
        // File::options()
        //     .read(true)
        //     .write(true)
        //     .open(construct_path(&(&file.filename())))
        //     .and_then(|mut f| 
        //         f.write_all(file.contenu().as_bytes()))
        //     .context("Ca a explosé ")
        Ok(())
        
    }
    
    fn sujet(&self) -> String {
        self.filename.clone()
    }
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
    fn create(my_graph: &Graph) -> Result<(), ApplicationError> {
        let id =Uuid::new_v4();
        let query = "INSERT INTO graph (id, filename) VALUES (?1, ?2);";
        let connexion = database::opening_database().map_err(ApplicationError::from)?;


        info!("Adding new graph: {}", my_graph.filename);
        connexion.execute(query, (id.to_string(), my_graph.filename.clone()))?;

        File::create(construct_path(&(my_graph.filename()))).map_err(ApplicationError::from)?;
        Ok(())
    }


    fn update(graph: &Graph) -> Result<(), ApplicationError> {

        let id = graph.id;
        Self::get_one(&id)?;
        
        let ref_query = "UPDATE reference SET nom = ?1 WHERE id = ?3;";
        let connexion = database::opening_database()?;


        info!("Updating  graph: {}", graph.filename);
        connexion.execute(ref_query, (graph.filename.clone(), id.to_string()))?;
        Ok(())
    }


    fn delete(graph: &Uuid) -> Result<usize, ApplicationError> {
        info!("Start deleting: {}", &graph);
        database::opening_database()?
            .execute("DELETE FROM graph WHERE id=?1", [graph.to_string()])
            .map_err(ApplicationError::from)
    }


    fn get_all() -> Result<Vec<Graph>, ApplicationError> {
        let query = "SELECT g.id, g.filename
            FROM graph as g ;";
        Ok(database::opening_database()?
                    .prepare(query)?
                    .query_map([], map_row)?
                    .map(|row| row.unwrap())
                    .collect::<Vec<Graph>>())
    }


    fn get_one(id: &Uuid) -> Result<Graph, ApplicationError> {
        let query = "SELECT g.id, g.filename
            FROM graph as g 
            WHERE g.id = :id 
            LIMIT 1;";
        database::opening_database()?
                .prepare(query)?
                .query_map([id.to_string()], map_row)?
                .next()
                .transpose()?
                .ok_or_else(||ApplicationError::DefaultError("expection something".to_string()))
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

    pub fn filename(&self) -> String {
        self.filename.clone() + ".dot"
    }
    
    pub  fn load_graph(&self) -> Result<DotGraph, ApplicationError> {
        DotGraph::graph_from_file(&construct_path(&self.filename())).map_err(|e| ApplicationError::DefaultError("While parsing".to_string()))
     }

     pub fn contenu(&self)->Result<String, io::Error> {
        let filename= self.filename();
        read_to_string(construct_path(&filename))
     }
}

 
