use std::collections::HashMap;

use egui::{emath::Numeric, Pos2};
use petgraph::{prelude::StableGraph, visit::{EdgeRef, IntoEdgeReferences}};
use rand::Rng;
use ilmen_dot_parser::Node as DotNode;

#[derive(Clone, Default)]
pub struct GuiNode(pub DotNode,pub Pos2);

impl From<&DotNode> for GuiNode {
    fn from(value: &DotNode) -> Self {
        Self(value.clone(), node_to_pos(value))
    }
}

fn node_to_pos(node: &DotNode) -> Pos2 {
    let mut rng = rand::rng();
        
    let rand = rng.random_range(0..100);

    let x = f32::from_f64(rand.to_f64().cos() * 50.0);
    let y = f32::from_f64(rand.to_f64().sin() * 50.0);
    Pos2::new(x, y)
}
pub struct GuiGraph(pub StableGraph<GuiNode, String>);

impl From<StableGraph<DotNode, String>> for GuiGraph {
    fn from(value: StableGraph<DotNode, String>) -> Self {
        let mut graph = StableGraph::<GuiNode, String>::new();
    
        let mut map_vertex_indice: HashMap<String, petgraph::prelude::NodeIndex> = HashMap::default();

        value.node_weights().collect::<Vec<&DotNode>>()
            .iter()
            .for_each(|n| {
                let index = graph.add_node(GuiNode::from(*n));
                map_vertex_indice.insert(n.identifier.clone(), index);
            });

        value.edge_references().for_each(|edge| {
            graph.add_edge(edge.source(), edge.target(), String::default());
            let source_node_pos = graph.node_weight(edge.source()).expect("Source node").1;
            let mut rng = rand::rng();
                    
            let rand = rng.random_range(0..100);

            let new_x_pos = f32::from_f64(source_node_pos.x.to_f64() + (rand.to_f64().cos() * value.node_count().to_f64().max(5.0) * 4.));
            let new_y_pos = f32::from_f64(source_node_pos.y.to_f64() + (rand.to_f64().sin() * value.node_count().to_f64().max(5.0) * 4.));

            graph.node_weight_mut(edge.target()).expect("Should have a node").1 = Pos2::new(new_x_pos, new_y_pos);
        });
        GuiGraph(graph)
    }
}