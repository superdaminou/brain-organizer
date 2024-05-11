use std::fs::read_to_string;

use log::info;

use crate::application::{error::ApplicationError, file::lib::{NODES_FILE, RELATIONS_FILE}, graph::{self, structs::{MyNode, Relations}}, reference::structs::reference::CsvLine};
use anyhow::{Context, Result};
const IMPORT_STORAGE: &str = "./import/";


pub fn import() -> Result<(), ApplicationError> {
    // info!("Start importing reference file: {}", REFERENCE_FILE);
    // read_to_string(IMPORT_STORAGE.to_string() + REFERENCE_FILE).context("Reading file")?
    //     .lines()  // split the string into an iterator of string slices
    //     .map(Reference::try_from)
    //     .collect::<Result<Vec<Reference>, ApplicationError>>()?
    //     .iter()
    //     .try_for_each(reference::service::create)?;

    // info!("Start importing reflexion file: {}", REFLEXION_FILE);
    // read_to_string(IMPORT_STORAGE.to_string() + REFLEXION_FILE).context("Read file")?  
    //     .lines()  // split the string into an iterator of string slices
    //     .map(CsvLine::from)
    //     .map(Reflexion::try_from)
    //     .collect::<Result<Vec<Reflexion>, ApplicationError>>()?
    //     .iter()
    //     .try_for_each(reflexion::service::create)?;

    
    // copy_recursively(IMPORT_STORAGE.to_string() + REFLEXION_STORAGE, REFLEXION_STORAGE).map_err(ApplicationError::Other)?;
    
    import_nodes()?;
    import_relations()
}

fn import_nodes() -> Result<(), ApplicationError> {
    info!("Start importing reflexion file: {}", NODES_FILE);
    let nodes =  read_to_string(IMPORT_STORAGE.to_string() + NODES_FILE).context("Reading file")?  
        .lines()  // split the string into an iterator of string slices
        .map(CsvLine::from)
        .map(MyNode::try_from)
        .collect::<Result<Vec<MyNode>, ApplicationError>>()?;
        graph::lib::save_nodes(nodes)?;
        Ok(())

}

fn import_relations() -> Result<(), ApplicationError> {
    info!("Start importing reflexion file: {}", RELATIONS_FILE);
    let relations =  read_to_string(IMPORT_STORAGE.to_string() + RELATIONS_FILE).context("Reading file")?  
        .lines()  // split the string into an iterator of string slices
        .map(CsvLine::from)
        .map(|line |Relations::try_from(&line))
        .collect::<Result<Vec<Relations>, ApplicationError>>()?;
        
        graph::lib::save_relations(relations)?;
        Ok(())
}   