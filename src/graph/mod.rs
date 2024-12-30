use my_graph::Graph;

use crate::application_error::ApplicationError;

pub mod my_graph;
mod connecteur;

pub trait ConnecteurGraph {
    fn get_one(&self, id: &String) -> Result<Graph, ApplicationError>;
    fn get_all(&self) -> Result<Vec<Graph>, ApplicationError>;
    fn delete(&self, note: &String) -> Result<(), ApplicationError>;
    fn create(&self, note: &Graph) -> Result<(), ApplicationError>;
    fn update(&self, note: &Graph) -> Result<(), ApplicationError>;
}