use std::fs::read_to_string;

use log::{error, info};

use crate::application::{database::CRUD, error::ApplicationError, file::lib::{copy_recursively, NODES_FILE, REFERENCE_FILE, REFLEXION_FILE, REFLEXION_STORAGE, RELATIONS_FILE}, graph::{lib::{Graph, GraphDatabase}, structs::{my_node::MyNode, relation::Relations}}, reference::structs::reference::Reference, reflexion::{service::ReflexionDatabase, Reflexion}};
use anyhow::{Context,Result};
const IMPORT_STORAGE: &str = "./import/";


pub fn import() -> Result<(), ApplicationError> {
    import_reference()
    .and_then(|_| import_reflexion())
    .and_then(|_| import_nodes())
    .and_then(|_| import_relations())
}

fn import_reference() -> Result<(), ApplicationError> {
    info!("Start importing reference file: {}", REFERENCE_FILE);
    read_to_string(IMPORT_STORAGE.to_string() + REFERENCE_FILE).context("Reading file")?
        .lines()  
        .map(Reference::try_from)
        .collect::<Result<Vec<Reference>, ApplicationError>>()?
        .iter()
        .map(Reference::create)
        .for_each(|result| {
            match result {
                Ok(()) => (),
                Err(e) => error!("error while inserting reference: {}", e)
            }
        });
    Ok(())
}

fn import_reflexion() -> Result<(), ApplicationError> {
    info!("Start importing reflexion file: {}", REFLEXION_FILE);
    read_to_string(IMPORT_STORAGE.to_string() + REFLEXION_FILE).context("Read file")?  
        .lines()  
        .map(Reflexion::try_from)
        .collect::<Result<Vec<Reflexion>, ApplicationError>>()?
        .iter()
        .map(Reflexion::create)
        .for_each(|result| {
            match result {
                Ok(()) => (),
                Err(e) => error!("error while inserting reflexion: {}", e)
            }
        });
    copy_recursively(IMPORT_STORAGE.to_string() + REFLEXION_STORAGE, REFLEXION_STORAGE).map_err(ApplicationError::Other)
}


fn import_nodes() -> Result<(), ApplicationError> {
    info!("Start importing nodes file: {}", NODES_FILE);
    read_to_string(IMPORT_STORAGE.to_string() + NODES_FILE)
        .with_context(|| format!("Reading file {}", NODES_FILE))?  
        .lines() 
        .map(MyNode::try_from)
        .collect::<Result<Vec<MyNode>, ApplicationError>>()
        .and_then(Graph::save_nodes)

}

fn import_relations() -> Result<(), ApplicationError> {
    info!("Start importing relations file: {}", RELATIONS_FILE);
    read_to_string(IMPORT_STORAGE.to_string() + RELATIONS_FILE)
        .with_context(|| format!("Reading file {}", NODES_FILE))?  
        .lines()  // split the string into an iterator of string slices
        .map(Relations::try_from)
        .collect::<Result<Vec<Relations>, ApplicationError>>()
        .and_then(Graph::save_relations)
}   