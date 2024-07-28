use crate::application::error::ApplicationError;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GraphType {
    Graph,
    Digraph
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
