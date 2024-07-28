use crate::application::error::ApplicationError;

use super::dot_graph::DotGraph;


#[derive(PartialEq, Eq)]
pub enum GraphFamily {
    Subgraph,
    Graph(GraphType)
} 

#[derive(PartialEq, Eq)]
pub enum GraphType {
    Graph,
    Digraph
}
enum Declaration {
    Node(String),
    Edge(String),
    Attribut(String),
}

enum DotType {
    Node(String),
    Edge(String),
    DotGraph(DotGraph),
    Declaration(String)
}

impl TryFrom<&str> for DotType {
    type Error = ApplicationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "node" => Ok(DotType::Node("Node".to_string())),
            "edge" => Ok(DotType::Edge("Edge".to_string())),
            other => {
                let dot_type = GraphFamily::try_from(other)
                    .map(|dot_graph|DotType::DotGraph(DotGraph::default()))
                    .unwrap_or(DotType::Declaration(other.to_string()));
                Ok(dot_type)
            }
        }
    }
} 



impl TryFrom<&str> for GraphType {
    type Error = ApplicationError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "digraph" => Ok(GraphType::Digraph),
            "graph" => Ok(GraphType::Graph),
            other => Err(ApplicationError::EnumError(other.to_string()))
        }
    }
}

impl TryFrom<&str> for GraphFamily {
    type Error = ApplicationError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "subgraph" => Ok(Self::Subgraph),
            other => Ok(Self::Graph(GraphType::try_from(other)?))
        }
    }
}