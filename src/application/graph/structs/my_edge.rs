use crate::application::error::ApplicationError;
use indradb::Edge;

use super::edge_type::Type;

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