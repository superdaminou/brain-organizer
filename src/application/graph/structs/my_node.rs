use egui_graphs::Node as EguiNode;
use indradb::{Identifier, Vertex, VertexProperties};
use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use uuid::Uuid;

use crate::application::{error::ApplicationError, graph::lib::IDENTIFIER, reference::structs::reference::CsvLine};

use super::my_edge::MyEdge;


#[derive(Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct MyNode {
    pub id: Option<Uuid>,
    pub node_type: NodeType,
    pub identifier: String
}

impl Default for MyNode {
    fn default() -> Self {
        Self { 
            id: None,
            node_type: NodeType::Autre,
            identifier: String::from("Default Node")
         }
    }
}


impl TryFrom<CsvLine> for MyNode {
    type Error = ApplicationError;
    fn try_from(value: CsvLine) -> Result<Self, ApplicationError> {
        let split: Vec<_> = value.split(';').collect();
        Ok(Self {
            id: None,
            node_type: NodeType::Autre,
            identifier: split.get(1).expect("Missing identifier").trim().to_string()
        })
    }
}

impl From<&Vertex> for MyNode {
    fn from(value: &Vertex) -> Self {
        info!("{}", value.id);
        MyNode {
            id: Some(value.id),
            node_type: NodeType::Autre,
            identifier: value.t.to_string()
        }
    }
}

impl From<&VertexProperties> for MyNode {
    fn from(value: &VertexProperties) -> Self {
        let a = value.props.iter()
        .find(|p| p.name == Identifier::new(IDENTIFIER).unwrap())
        .and_then(|named| named.value.as_str())
        .unwrap_or("DefaultValue");

        MyNode {
            id: Some(value.vertex.id),
            node_type: NodeType::Autre,
            identifier: a.to_string()
        }
    }
}


impl From<&EguiNode<MyNode, MyEdge>> for MyNode {
    fn from(value: &EguiNode<MyNode, MyEdge>) -> Self {
        MyNode {
            id: value.payload().id,
            node_type: NodeType::Autre,
            identifier: value.label()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize,  EnumIter, Display, Hash,Copy, Ord, PartialOrd)]
pub enum NodeType {
    Concept,
    Personne,
    Lieu,
    Evenement,
    Autre
}

