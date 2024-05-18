use log::info;

use crate::application::{error::ApplicationError, graph::lib::get_node, reference::structs::reference::CsvLine};

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
        let node_out = get_node(&split.first().expect("Wesh").trim().to_string())?;
        info!("{}", &node_out.identifier);
        let node_in = get_node(&split.get(2).expect("Wesh").trim().to_string())?;
        info!("{}", &node_in.identifier);
        let edge= split.get(1)
            .ok_or(ApplicationError::DefaultError)
            .map(|s|MyEdge::try_from(s.to_string()))??;
        info!("{}", &edge.edge_type.identifier());
        Ok(Relations{node_out , edge,node_in})
    }

}



