use petgraph::stable_graph::StableGraph;

use self::{my_edge::MyEdge, my_node::MyNode};

pub mod my_edge;
pub mod my_node;
pub mod edge_type;
pub mod relation;


pub struct NodeGraph(pub StableGraph<MyNode, MyEdge>);