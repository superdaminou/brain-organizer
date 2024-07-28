use std::str::FromStr;

use egui::TextBuffer;
use indradb::{Identifier, Vertex, VertexProperties};
use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use uuid::Uuid;

use crate::application::{error::ApplicationError, graph::lib::{IDENTIFIER, NODE_TYPE}, reference::structs::reference::CsvLine};



#[derive(Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct MyNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub identifier: String
}

impl Default for MyNode {
    fn default() -> Self {
        Self { 
            id: Uuid::new_v4(),
            node_type: NodeType::Autre,
            identifier: String::from("Default Node")
         }
    }
}

impl TryFrom<&CsvLine> for MyNode {
    type Error = ApplicationError;
    fn try_from(value: &CsvLine) -> Result<Self, ApplicationError> {
        let split: Vec<_> = value.split(';').collect();
        Ok(Self {
            id: Uuid::new_v4(),
            node_type: NodeType::from_str(split.first().expect("Missing Node type").as_str())?,
            identifier: split.get(1).expect("Missing identifier").trim().to_string()
        })
    }
}

impl From<&Vertex> for MyNode {
    fn from(value: &Vertex) -> Self {
        info!("{}", value.id);
        MyNode {
            id: value.id,
            node_type: NodeType::Autre,
            identifier: value.t.to_string()
        }
    }
}

impl From<&VertexProperties> for MyNode {
    fn from(value: &VertexProperties) -> Self {
        let identifier = value.props.iter()
        .find(|p| p.name == Identifier::new(IDENTIFIER).unwrap())
        .and_then(|named| named.value.as_str())
        .unwrap_or("DefaultValue");

        let node_type = value.props.iter()
        .find(|p| p.name == Identifier::new(NODE_TYPE).unwrap()).map(|named| named.value.to_string())
        .map(|str| NodeType::from_str(str.as_str()).unwrap_or(NodeType::Autre))
        .unwrap_or(NodeType::Autre);

        MyNode {
            id: value.vertex.id,
            node_type,
            identifier: identifier.to_string()
        }
    }
}




#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize,  EnumIter, Display, Hash,Copy, Ord, PartialOrd)]
pub enum NodeType {
    Concept,
    Personne,
    Lieu,
    Evenement,
    Autre,
    Periode
}

impl FromStr for NodeType {
    type Err = ApplicationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value: Result<NodeType, ApplicationError> = match value.to_lowercase().as_str() {
            "concept" => Ok(NodeType::Concept),
            "personne" => Ok(NodeType::Personne),
            "lieu" => Ok(NodeType::Lieu),
            "ville" => Ok(NodeType::Lieu),
            "evenement" => Ok(NodeType::Evenement),
            "autre" => Ok(NodeType::Autre),
            "periode" => Ok(NodeType::Periode),
            _ => Err(ApplicationError::EnumError(value.to_string()))
        };
        value
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::application::graph::structs::my_node::NodeType;

    #[test]
    fn try_from_unknown_identifier() {
        let result = NodeType::from_str("unexpected");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Could not determine enum from: unexpected" )
    }

}