use crate::application::{error::ApplicationError, graph::lib::{Graph, GraphDatabase}, reference::structs::reference::CsvLine};

use super::{my_edge::MyEdge, my_node::MyNode};



pub struct Relations{
    pub node_out: MyNode,
    pub edge: MyEdge,
    pub node_in: MyNode
}


impl TryFrom<&CsvLine> for Relations {
    type Error = ApplicationError;
    fn try_from(line: &CsvLine) -> Result<Relations, Self::Error> {
        let split : Vec<_>= line.split(';').collect();
        
        let node_out = Graph::get_node(split.first().expect("Expecting node").trim())?;
        let node_in = Graph::get_node(split.get(2).expect("Expecting node").trim())?;
        let edge= split.get(1)
            .ok_or(ApplicationError::DefaultError("Not found".to_string()))
            .map(|s|MyEdge::try_from(s.to_string()))??;
        Ok(Relations{node_out , edge,node_in})
    }

}



