use reqwest::blocking::Body;
use crate::{application_error::ApplicationError, client, graph::{my_graph::Graph, ConnecteurGraph}};


pub struct ConnecteurWebGraph;

impl ConnecteurWebGraph {
    pub fn new() -> ConnecteurWebGraph {
        ConnecteurWebGraph
    }
}

impl ConnecteurGraph for ConnecteurWebGraph {
    fn get_one(&self, id: &String) -> Result<Graph, ApplicationError> {
        let path = format!("/graphs/{}", id);
        client::get(&path)?
            .json::<Graph>()
            .map_err(ApplicationError::from)
    }

    fn get_all(&self) -> Result<Vec<Graph>, ApplicationError> {
        let path = "/graphs".to_string();
        client::get(&path)?
            .json::<Vec<Graph>>()
            .map_err(ApplicationError::from)
    }

    fn delete(&self, graph: &String) -> Result<(), ApplicationError> {
        let path = format!("/graphs/{}", graph);
        client::delete(&path)?
            .json::<()>()
            .map_err(ApplicationError::from)
    }

    fn create(&self, graph: &Graph) -> Result<(), ApplicationError> {
        let path = "/graphs".to_string();
        client::post(&path, Body::from(serde_json::to_string(graph).unwrap()));
        Ok(())
    }

    fn update(&self, graph: &Graph) -> Result<(), ApplicationError> {
        let path = format!("/graphs/{}", graph.id);
        client::update(&path, Body::from(serde_json::to_string(graph).unwrap()))?
            .json::<()>()
            .map_err(|e|ApplicationError::DefaultError(e.to_string()))
    }
}