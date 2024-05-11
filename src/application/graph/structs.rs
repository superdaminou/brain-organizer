use core::fmt;
use egui_graphs::Node as EguiNode;
use indradb::{Edge, Identifier, Vertex, VertexProperties};
use log::info;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use uuid::Uuid;

use crate::application::{error::ApplicationError, reference::structs::reference::CsvLine};

use super::lib::{get_node, IDENTIFIER};

#[derive(Clone)]
pub struct MyNode {
    pub id: Option<Uuid>,
    pub node_type: String,
    pub identifier: String
}

impl Default for MyNode {
    fn default() -> Self {
        Self { 
            id: None,
            node_type: String::from("Node"),
            identifier: String::from("Default Node")
         }
    }
}


impl From<&String> for MyNode {
    fn from(value: &String) -> Self {
        Self {
            id: None,
            node_type: String::from("Concept"), 
            identifier: value.clone()
        }
    }
}

impl TryFrom<CsvLine> for MyNode {
    type Error = ApplicationError;
    fn try_from(value: CsvLine) -> Result<Self, ApplicationError> {
        let split: Vec<_> = value.split(';').collect();
        Ok(Self {
            id: None,
            node_type: split.first().expect("Missing Node Type").trim().to_string(),
            identifier: split.get(1).expect("Missing identifier").trim().to_string()
        })
    }
}

impl From<&Vertex> for MyNode {
    fn from(value: &Vertex) -> Self {
        MyNode {
            id: Some(value.id),
            node_type: String::from("Node"),
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
            node_type: String::from("Node"),
            identifier: a.to_string()
        }
    }
}


impl From<&EguiNode<MyNode, MyEdge>> for MyNode {
    fn from(value: &EguiNode<MyNode, MyEdge>) -> Self {
        MyNode {
            id: None,
            node_type: String::from("Node"),
            identifier: value.label()
        }
    }
}


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
        let r = Relations{node_out , edge,node_in};
        Ok(r)
    }

}



#[derive(Clone)]
pub struct MyEdge {
    pub edge_type: Type
}

impl Default for MyEdge {
    fn default() -> Self {
        Self { edge_type: Type::Definie }
    }
}

impl From<Type> for MyEdge {
    fn from(value: Type) -> Self {
        MyEdge {
            edge_type: value
        }
    }
}

impl TryFrom<&Edge> for MyEdge {
    type Error = ApplicationError;
    fn try_from(edge: &Edge) -> Result<MyEdge, Self::Error> {
        Ok(MyEdge {
            edge_type: Type::try_from(edge.t)?
        })
    }

}

impl TryFrom<String> for MyEdge {
    type Error = ApplicationError;
    fn try_from(_edge_type: String) -> Result<MyEdge, Self::Error> {
        Ok(MyEdge {
            edge_type: Type::Definie
        })
    }

}



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize,  EnumIter)]
pub enum Type {
    ALieuA,
    Definie,
}

impl TryFrom<Identifier> for Type {
    type Error = ApplicationError;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        let value: Result<Type, ApplicationError> = match value.to_lowercase().as_str() {
            "definie" => Ok(Type::Definie),
            "alieua" => Ok(Type::ALieuA),
            _ => Err(ApplicationError::DefaultError)
        };
        value
    }
}

impl Type {
    pub fn identifier(&self) -> &'static str {
        match self {
            Type::ALieuA => "a_lieu_a",
            Type::Definie => "definie"
        }
    }
}



impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::ALieuA => write!(f, "A eu lieu a "),
            Type::Definie => write!(f,"Définis"),
        }
    }
}
