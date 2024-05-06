use core::fmt;

use egui_graphs::Node as EguiNode;
use indradb::{Edge, Identifier, Vertex, VertexProperties};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::application::error::ApplicationError;

use super::lib::PERSONNE_IDENTIFIER;

#[derive(Clone)]
pub struct MyNode {
    pub name: String
}

impl Default for MyNode {
    fn default() -> Self {
        Self { name: String::from("Default Node") }
    }
}


impl From<&String> for MyNode {
    fn from(value: &String) -> Self {
        Self { name: value.clone()}
    }
}

impl From<&Vertex> for MyNode {
    fn from(value: &Vertex) -> Self {
        MyNode {
            name: value.t.to_string()
        }
    }
}

impl From<&VertexProperties> for MyNode {
    fn from(value: &VertexProperties) -> Self {
        let a = value.props.iter()
        .find(|p| p.name == Identifier::new(PERSONNE_IDENTIFIER).unwrap())
        .and_then(|named| named.value.as_str())
        .unwrap_or("DefaultValue");

        MyNode {
            name: a.to_string()
        }
    }
}


impl From<&EguiNode<MyNode, MyEdge>> for MyNode {
    fn from(value: &EguiNode<MyNode, MyEdge>) -> Self {
        MyNode {
            name: value.label()
        }
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

impl TryFrom<&Edge> for MyEdge {
    type Error = ApplicationError;
    fn try_from(edge: &Edge) -> Result<MyEdge, Self::Error> {
        Ok(MyEdge {
            edge_type: Type::try_from(edge.t)?
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
