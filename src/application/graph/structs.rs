use core::fmt;

use egui_graphs::Node as EguiNode;
use indradb::{Edge, Vertex};
use serde::Serialize;

use crate::application::error::ApplicationError;

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


impl From<&EguiNode<MyNode, MyEdge>> for MyNode {
    fn from(value: &EguiNode<MyNode, MyEdge>) -> Self {
        MyNode {
            name: String::from(value.label())
        }
    }
}




#[derive(Clone)]
pub struct MyEdge {
    pub edge_type: Type
}

impl Default for MyEdge {
    fn default() -> Self {
        Self { edge_type: Type::DEFINIE }
    }
}

impl TryFrom<&Edge> for MyEdge {
    type Error = ApplicationError;
    fn try_from(edge: &Edge) -> Result<MyEdge, Self::Error> {
        Ok(MyEdge {
            edge_type: Type::DEFINIE
        })
    }

}


#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Type {
    ALieuA,
    DEFINIE,
}



impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::ALieuA => write!(f, "A eu lieu a "),
            Type::DEFINIE => write!(f,"Définis"),
        }
    }
}
